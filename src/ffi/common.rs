//! Bindings to `hb-common.h`.

#![allow(non_camel_case_types)]

use std::os::raw::*;

pub type hb_bool_t = c_int;
pub type hb_codepoint_t = u32;
pub type hb_position_t = i32;
pub type hb_mask_t = u32;

#[derive(Clone, Copy)]
#[repr(C)]
pub union hb_var_int_t {
    u32: u32,
    i32: i32,
    u16: [u16; 2],
    i16: [i16; 2],
    u8: [u8; 4],
    i8: [i8; 4],
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct hb_tag_t(u32);

impl hb_tag_t {
    #[inline]
    pub const fn new(tag: [u8; 4]) -> hb_tag_t {
        hb_tag_t(
            (tag[0] as u32) << 24 |
            (tag[1] as u32) << 16 |
            (tag[2] as u32) << 8 |
            (tag[3] as u32)
        )
    }

    /*pub fn from_string(str: &str) -> hb_tag_t {
        let bytes = str.as_bytes();
        unsafe {
            hb_tag_from_string(bytes.as_ptr() as *const c_char, bytes.len() as c_int)
        }
    }*/
}

/*impl fmt::Debug for hb_tag_t {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = [0u8; 4];
        unsafe {
            hb_tag_to_string(*self, buf.as_mut_ptr() as *mut c_char);
        }
        match str::from_utf8(&buf) {
            Ok(s) => write!(fmt, "hb_tag_t({})", s),
            Err(_) => write!(fmt, "hb_tag_t({:?})", buf),
        }
    }
}
*/
extern "C" {
    pub fn hb_tag_from_string(str: *const c_char, len: c_int) -> hb_tag_t;
    pub fn hb_tag_to_string(tag: hb_tag_t, buf: *mut c_char) -> c_void;
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum hb_direction_t {
    /// Initial, unset direction.
    HB_DIRECTION_INVALID = 0,
    /// Text is set horizontally from left to right.
    HB_DIRECTION_LTR = 4,
    /// Text is set horizontally from right to left.
    HB_DIRECTION_RTL,
    /// Text is set vertically from top to bottom.
    HB_DIRECTION_TTB,
    /// Text is set vertically from bottom to top.
    HB_DIRECTION_BTT
}

extern "C" {
    pub fn hb_direction_from_string(str: *const c_char, len: c_int) -> hb_direction_t;
    pub fn hb_direction_to_string(direction: hb_direction_t) -> *const c_char;
}

#[repr(C)]
pub struct hb_language_impl_t(cffi::Private);

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct hb_language_t(*const hb_language_impl_t);

/*impl fmt::Debug for hb_language_t {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let s = unsafe {
            CStr::from_ptr(hb_language_to_string(*self))
        };
        match s.to_str() {
            Ok(s) => fmt::Debug::fmt(s, fmt),
            Err(_) => fmt.write_str("<invalid utf-8 sequence in language tag>"),
        }
    }
}*/

extern "C" {
    pub fn hb_language_from_string(str: *const c_char, len: c_int) -> hb_language_t;
    pub fn hb_language_to_string(language: hb_language_t) -> *const c_char;
    pub fn hb_language_get_default() -> hb_language_t;
}

#[derive(Clone, Copy/*, Debug*/)]
#[repr(C)]
pub struct hb_script_t(hb_tag_t);

/*impl hb_script_t {
    #[inline]
    pub fn from_str(str: &str) -> hb_script_t {
        let bytes = str.as_bytes();
        unsafe {
            hb_script_from_string(bytes.as_ptr() as *const c_char, bytes.len() as c_int)
        }
    }

    #[inline]
    pub fn from_iso15924_tag(tag: hb_tag_t) -> hb_script_t {
        unsafe { hb_script_from_iso15924_tag(tag) }
    }

    #[inline]
    pub fn to_iso15924_tag(self) -> hb_tag_t {
        unsafe { hb_script_to_iso15924_tag(self) }
    }

    #[inline]
    pub fn get_horizontal_direction(self) -> hb_direction_t {
        unsafe { hb_script_get_horizontal_direction(self) }
    }
}*/

extern "C" {
    pub fn hb_script_from_iso15924_tag(tag: hb_tag_t) -> hb_script_t;
    pub fn hb_script_from_string(str: *const c_char, len: c_int) -> hb_script_t;
    pub fn hb_script_to_iso15924_tag(script: hb_script_t) -> hb_tag_t;
    pub fn hb_script_get_horizontal_direction(script: hb_script_t) -> hb_direction_t;
}

pub const SCRIPT_COMMON: hb_script_t = hb_script_t(hb_tag_t::new(*b"Zyyy"));
pub const SCRIPT_INHERITED: hb_script_t = hb_script_t(hb_tag_t::new(*b"Zinh"));
pub const SCRIPT_UNKNOWN: hb_script_t = hb_script_t(hb_tag_t::new(*b"Zzzz"));
pub const SCRIPT_ARABIC: hb_script_t = hb_script_t(hb_tag_t::new(*b"Arab"));
pub const SCRIPT_ARMENIAN: hb_script_t = hb_script_t(hb_tag_t::new(*b"Armn"));
pub const SCRIPT_BENGALI: hb_script_t = hb_script_t(hb_tag_t::new(*b"Beng"));
pub const SCRIPT_CYRILLIC: hb_script_t = hb_script_t(hb_tag_t::new(*b"Cyrl"));
pub const SCRIPT_DEVANAGARI: hb_script_t = hb_script_t(hb_tag_t::new(*b"Deva"));
pub const SCRIPT_GEORGIAN: hb_script_t = hb_script_t(hb_tag_t::new(*b"Geor"));
pub const SCRIPT_GREEK: hb_script_t = hb_script_t(hb_tag_t::new(*b"Grek"));
pub const SCRIPT_GUJARATI: hb_script_t = hb_script_t(hb_tag_t::new(*b"Gujr"));
pub const SCRIPT_GURMUKHI: hb_script_t = hb_script_t(hb_tag_t::new(*b"Guru"));
pub const SCRIPT_HANGUL: hb_script_t = hb_script_t(hb_tag_t::new(*b"Hang"));
pub const SCRIPT_HAN: hb_script_t = hb_script_t(hb_tag_t::new(*b"Hani"));
pub const SCRIPT_HEBREW: hb_script_t = hb_script_t(hb_tag_t::new(*b"Hebr"));
pub const SCRIPT_HIRAGANA: hb_script_t = hb_script_t(hb_tag_t::new(*b"Hira"));
pub const SCRIPT_KANNADA: hb_script_t = hb_script_t(hb_tag_t::new(*b"Knda"));
pub const SCRIPT_KATAKANA: hb_script_t = hb_script_t(hb_tag_t::new(*b"Kana"));
pub const SCRIPT_LAO: hb_script_t = hb_script_t(hb_tag_t::new(*b"Laoo"));
pub const SCRIPT_LATIN: hb_script_t = hb_script_t(hb_tag_t::new(*b"Latn"));
pub const SCRIPT_MALAYALAM: hb_script_t = hb_script_t(hb_tag_t::new(*b"Mlym"));
pub const SCRIPT_ORIYA: hb_script_t = hb_script_t(hb_tag_t::new(*b"Orya"));
pub const SCRIPT_TAMIL: hb_script_t = hb_script_t(hb_tag_t::new(*b"Taml"));
pub const SCRIPT_TELUGU: hb_script_t = hb_script_t(hb_tag_t::new(*b"Telu"));
pub const SCRIPT_THAI: hb_script_t = hb_script_t(hb_tag_t::new(*b"Thai"));
pub const SCRIPT_TIBETAN: hb_script_t = hb_script_t(hb_tag_t::new(*b"Tibt"));
pub const SCRIPT_BOPOMOFO: hb_script_t = hb_script_t(hb_tag_t::new(*b"Bopo"));
pub const SCRIPT_BRAILLE: hb_script_t = hb_script_t(hb_tag_t::new(*b"Brai"));
pub const SCRIPT_CANADIAN_SYLLABICS: hb_script_t = hb_script_t(hb_tag_t::new(*b"Cans"));
pub const SCRIPT_CHEROKEE: hb_script_t = hb_script_t(hb_tag_t::new(*b"Cher"));
pub const SCRIPT_ETHIOPIC: hb_script_t = hb_script_t(hb_tag_t::new(*b"Ethi"));
pub const SCRIPT_KHMER: hb_script_t = hb_script_t(hb_tag_t::new(*b"Khmr"));
pub const SCRIPT_MONGOLIAN: hb_script_t = hb_script_t(hb_tag_t::new(*b"Mong"));
pub const SCRIPT_MYANMAR: hb_script_t = hb_script_t(hb_tag_t::new(*b"Mymr"));
pub const SCRIPT_OGHAM: hb_script_t = hb_script_t(hb_tag_t::new(*b"Ogam"));
pub const SCRIPT_RUNIC: hb_script_t = hb_script_t(hb_tag_t::new(*b"Runr"));
pub const SCRIPT_SINHALA: hb_script_t = hb_script_t(hb_tag_t::new(*b"Sinh"));
pub const SCRIPT_SYRIAC: hb_script_t = hb_script_t(hb_tag_t::new(*b"Syrc"));
pub const SCRIPT_THAANA: hb_script_t = hb_script_t(hb_tag_t::new(*b"Thaa"));
pub const SCRIPT_YI: hb_script_t = hb_script_t(hb_tag_t::new(*b"Yiii"));
pub const SCRIPT_DESERET: hb_script_t = hb_script_t(hb_tag_t::new(*b"Dsrt"));
pub const SCRIPT_GOTHIC: hb_script_t = hb_script_t(hb_tag_t::new(*b"Goth"));
pub const SCRIPT_OLD_ITALIC: hb_script_t = hb_script_t(hb_tag_t::new(*b"Ital"));
pub const SCRIPT_BUHID: hb_script_t = hb_script_t(hb_tag_t::new(*b"Buhd"));
pub const SCRIPT_HANUNOO: hb_script_t = hb_script_t(hb_tag_t::new(*b"Hano"));
pub const SCRIPT_TAGALOG: hb_script_t = hb_script_t(hb_tag_t::new(*b"Tglg"));
pub const SCRIPT_TAGBANWA: hb_script_t = hb_script_t(hb_tag_t::new(*b"Tagb"));
pub const SCRIPT_CYPRIOT: hb_script_t = hb_script_t(hb_tag_t::new(*b"Cprt"));
pub const SCRIPT_LIMBU: hb_script_t = hb_script_t(hb_tag_t::new(*b"Limb"));
pub const SCRIPT_LINEAR_B: hb_script_t = hb_script_t(hb_tag_t::new(*b"Linb"));
pub const SCRIPT_OSMANYA: hb_script_t = hb_script_t(hb_tag_t::new(*b"Osma"));
pub const SCRIPT_SHAVIAN: hb_script_t = hb_script_t(hb_tag_t::new(*b"Shaw"));
pub const SCRIPT_TAI_LE: hb_script_t = hb_script_t(hb_tag_t::new(*b"Tale"));
pub const SCRIPT_UGARITIC: hb_script_t = hb_script_t(hb_tag_t::new(*b"Ugar"));
pub const SCRIPT_BUGINESE: hb_script_t = hb_script_t(hb_tag_t::new(*b"Bugi"));
pub const SCRIPT_COPTIC: hb_script_t = hb_script_t(hb_tag_t::new(*b"Copt"));
pub const SCRIPT_GLAGOLITIC: hb_script_t = hb_script_t(hb_tag_t::new(*b"Glag"));
pub const SCRIPT_KHAROSHTHI: hb_script_t = hb_script_t(hb_tag_t::new(*b"Khar"));
pub const SCRIPT_NEW_TAI_LUE: hb_script_t = hb_script_t(hb_tag_t::new(*b"Talu"));
pub const SCRIPT_OLD_PERSIAN: hb_script_t = hb_script_t(hb_tag_t::new(*b"Xpeo"));
pub const SCRIPT_SYLOTI_NAGRI: hb_script_t = hb_script_t(hb_tag_t::new(*b"Sylo"));
pub const SCRIPT_TIFINAGH: hb_script_t = hb_script_t(hb_tag_t::new(*b"Tfng"));
pub const SCRIPT_BALINESE: hb_script_t = hb_script_t(hb_tag_t::new(*b"Bali"));
pub const SCRIPT_CUNEIFORM: hb_script_t = hb_script_t(hb_tag_t::new(*b"Xsux"));
pub const SCRIPT_NKO: hb_script_t = hb_script_t(hb_tag_t::new(*b"Nkoo"));
pub const SCRIPT_PHAGS_PA: hb_script_t = hb_script_t(hb_tag_t::new(*b"Phag"));
pub const SCRIPT_PHOENICIAN: hb_script_t = hb_script_t(hb_tag_t::new(*b"Phnx"));
pub const SCRIPT_CARIAN: hb_script_t = hb_script_t(hb_tag_t::new(*b"Cari"));
pub const SCRIPT_CHAM: hb_script_t = hb_script_t(hb_tag_t::new(*b"Cham"));
pub const SCRIPT_KAYAH_LI: hb_script_t = hb_script_t(hb_tag_t::new(*b"Kali"));
pub const SCRIPT_LEPCHA: hb_script_t = hb_script_t(hb_tag_t::new(*b"Lepc"));
pub const SCRIPT_LYCIAN: hb_script_t = hb_script_t(hb_tag_t::new(*b"Lyci"));
pub const SCRIPT_LYDIAN: hb_script_t = hb_script_t(hb_tag_t::new(*b"Lydi"));
pub const SCRIPT_OL_CHIKI: hb_script_t = hb_script_t(hb_tag_t::new(*b"Olck"));
pub const SCRIPT_REJANG: hb_script_t = hb_script_t(hb_tag_t::new(*b"Rjng"));
pub const SCRIPT_SAURASHTRA: hb_script_t = hb_script_t(hb_tag_t::new(*b"Saur"));
pub const SCRIPT_SUNDANESE: hb_script_t = hb_script_t(hb_tag_t::new(*b"Sund"));
pub const SCRIPT_VAI: hb_script_t = hb_script_t(hb_tag_t::new(*b"Vaii"));
pub const SCRIPT_AVESTAN: hb_script_t = hb_script_t(hb_tag_t::new(*b"Avst"));
pub const SCRIPT_BAMUM: hb_script_t = hb_script_t(hb_tag_t::new(*b"Bamu"));
pub const SCRIPT_EGYPTIAN_HIEROGLYPHS: hb_script_t = hb_script_t(hb_tag_t::new(*b"Egyp"));
pub const SCRIPT_IMPERIAL_ARAMAIC: hb_script_t = hb_script_t(hb_tag_t::new(*b"Armi"));
pub const SCRIPT_INSCRIPTIONAL_PAHLAVI: hb_script_t = hb_script_t(hb_tag_t::new(*b"Phli"));
pub const SCRIPT_INSCRIPTIONAL_PARTHIAN: hb_script_t = hb_script_t(hb_tag_t::new(*b"Prti"));
pub const SCRIPT_JAVANESE: hb_script_t = hb_script_t(hb_tag_t::new(*b"Java"));
pub const SCRIPT_KAITHI: hb_script_t = hb_script_t(hb_tag_t::new(*b"Kthi"));
pub const SCRIPT_LISU: hb_script_t = hb_script_t(hb_tag_t::new(*b"Lisu"));
pub const SCRIPT_MEETEI_MAYEK: hb_script_t = hb_script_t(hb_tag_t::new(*b"Mtei"));
pub const SCRIPT_OLD_SOUTH_ARABIAN: hb_script_t = hb_script_t(hb_tag_t::new(*b"Sarb"));
pub const SCRIPT_OLD_TURKIC: hb_script_t = hb_script_t(hb_tag_t::new(*b"Orkh"));
pub const SCRIPT_SAMARITAN: hb_script_t = hb_script_t(hb_tag_t::new(*b"Samr"));
pub const SCRIPT_TAI_THAM: hb_script_t = hb_script_t(hb_tag_t::new(*b"Lana"));
pub const SCRIPT_TAI_VIET: hb_script_t = hb_script_t(hb_tag_t::new(*b"Tavt"));
pub const SCRIPT_BATAK: hb_script_t = hb_script_t(hb_tag_t::new(*b"Batk"));
pub const SCRIPT_BRAHMI: hb_script_t = hb_script_t(hb_tag_t::new(*b"Brah"));
pub const SCRIPT_MANDAIC: hb_script_t = hb_script_t(hb_tag_t::new(*b"Mand"));
pub const SCRIPT_CHAKMA: hb_script_t = hb_script_t(hb_tag_t::new(*b"Cakm"));
pub const SCRIPT_MEROITIC_CURSIVE: hb_script_t = hb_script_t(hb_tag_t::new(*b"Merc"));
pub const SCRIPT_MEROITIC_HIEROGLYPHS: hb_script_t = hb_script_t(hb_tag_t::new(*b"Mero"));
pub const SCRIPT_MIAO: hb_script_t = hb_script_t(hb_tag_t::new(*b"Plrd"));
pub const SCRIPT_SHARADA: hb_script_t = hb_script_t(hb_tag_t::new(*b"Shrd"));
pub const SCRIPT_SORA_SOMPENG: hb_script_t = hb_script_t(hb_tag_t::new(*b"Sora"));
pub const SCRIPT_TAKRI: hb_script_t = hb_script_t(hb_tag_t::new(*b"Takr"));
pub const SCRIPT_BASSA_VAH: hb_script_t = hb_script_t(hb_tag_t::new(*b"Bass"));
pub const SCRIPT_CAUCASIAN_ALBANIAN: hb_script_t = hb_script_t(hb_tag_t::new(*b"Aghb"));
pub const SCRIPT_DUPLOYAN: hb_script_t = hb_script_t(hb_tag_t::new(*b"Dupl"));
pub const SCRIPT_ELBASAN: hb_script_t = hb_script_t(hb_tag_t::new(*b"Elba"));
pub const SCRIPT_GRANTHA: hb_script_t = hb_script_t(hb_tag_t::new(*b"Gran"));
pub const SCRIPT_KHOJKI: hb_script_t = hb_script_t(hb_tag_t::new(*b"Khoj"));
pub const SCRIPT_KHUDAWADI: hb_script_t = hb_script_t(hb_tag_t::new(*b"Sind"));
pub const SCRIPT_LINEAR_A: hb_script_t = hb_script_t(hb_tag_t::new(*b"Lina"));
pub const SCRIPT_MAHAJANI: hb_script_t = hb_script_t(hb_tag_t::new(*b"Mahj"));
pub const SCRIPT_MANICHAEAN: hb_script_t = hb_script_t(hb_tag_t::new(*b"Mani"));
pub const SCRIPT_MENDE_KIKAKUI: hb_script_t = hb_script_t(hb_tag_t::new(*b"Mend"));
pub const SCRIPT_MODI: hb_script_t = hb_script_t(hb_tag_t::new(*b"Modi"));
pub const SCRIPT_MRO: hb_script_t = hb_script_t(hb_tag_t::new(*b"Mroo"));
pub const SCRIPT_NABATAEAN: hb_script_t = hb_script_t(hb_tag_t::new(*b"Nbat"));
pub const SCRIPT_OLD_NORTH_ARABIAN: hb_script_t = hb_script_t(hb_tag_t::new(*b"Narb"));
pub const SCRIPT_OLD_PERMIC: hb_script_t = hb_script_t(hb_tag_t::new(*b"Perm"));
pub const SCRIPT_PAHAWH_HMONG: hb_script_t = hb_script_t(hb_tag_t::new(*b"Hmng"));
pub const SCRIPT_PALMYRENE: hb_script_t = hb_script_t(hb_tag_t::new(*b"Palm"));
pub const SCRIPT_PAU_CIN_HAU: hb_script_t = hb_script_t(hb_tag_t::new(*b"Pauc"));
pub const SCRIPT_PSALTER_PAHLAVI: hb_script_t = hb_script_t(hb_tag_t::new(*b"Phlp"));
pub const SCRIPT_SIDDHAM: hb_script_t = hb_script_t(hb_tag_t::new(*b"Sidd"));
pub const SCRIPT_TIRHUTA: hb_script_t = hb_script_t(hb_tag_t::new(*b"Tirh"));
pub const SCRIPT_WARANG_CITI: hb_script_t = hb_script_t(hb_tag_t::new(*b"Wara"));
pub const SCRIPT_AHOM: hb_script_t = hb_script_t(hb_tag_t::new(*b"Ahom"));
pub const SCRIPT_ANATOLIAN_HIEROGLYPHS: hb_script_t = hb_script_t(hb_tag_t::new(*b"Hluw"));
pub const SCRIPT_HATRAN: hb_script_t = hb_script_t(hb_tag_t::new(*b"Hatr"));
pub const SCRIPT_MULTANI: hb_script_t = hb_script_t(hb_tag_t::new(*b"Mult"));
pub const SCRIPT_OLD_HUNGARIAN: hb_script_t = hb_script_t(hb_tag_t::new(*b"Hung"));
pub const SCRIPT_SIGNWRITING: hb_script_t = hb_script_t(hb_tag_t::new(*b"Sgnw"));
pub const SCRIPT_ADLAM: hb_script_t = hb_script_t(hb_tag_t::new(*b"Adlm"));
pub const SCRIPT_BHAIKSUKI: hb_script_t = hb_script_t(hb_tag_t::new(*b"Bhks"));
pub const SCRIPT_MARCHEN: hb_script_t = hb_script_t(hb_tag_t::new(*b"Marc"));
pub const SCRIPT_OSAGE: hb_script_t = hb_script_t(hb_tag_t::new(*b"Osge"));
pub const SCRIPT_TANGUT: hb_script_t = hb_script_t(hb_tag_t::new(*b"Tang"));
pub const SCRIPT_NEWA: hb_script_t = hb_script_t(hb_tag_t::new(*b"Newa"));
pub const SCRIPT_MASARAM_GONDI: hb_script_t = hb_script_t(hb_tag_t::new(*b"Gonm"));
pub const SCRIPT_NUSHU: hb_script_t = hb_script_t(hb_tag_t::new(*b"Nshu"));
pub const SCRIPT_SOYOMBO: hb_script_t = hb_script_t(hb_tag_t::new(*b"Soyo"));
pub const SCRIPT_ZANABAZAR_SQUARE: hb_script_t = hb_script_t(hb_tag_t::new(*b"Zanb"));
pub const SCRIPT_DOGRA: hb_script_t = hb_script_t(hb_tag_t::new(*b"Dogr"));
pub const SCRIPT_GUNJALA_GONDI: hb_script_t = hb_script_t(hb_tag_t::new(*b"Gong"));
pub const SCRIPT_HANIFI_ROHINGYA: hb_script_t = hb_script_t(hb_tag_t::new(*b"Rohg"));
pub const SCRIPT_MAKASAR: hb_script_t = hb_script_t(hb_tag_t::new(*b"Maka"));
pub const SCRIPT_MEDEFAIDRIN: hb_script_t = hb_script_t(hb_tag_t::new(*b"Medf"));
pub const SCRIPT_OLD_SOGDIAN: hb_script_t = hb_script_t(hb_tag_t::new(*b"Sogo"));
pub const SCRIPT_SOGDIAN: hb_script_t = hb_script_t(hb_tag_t::new(*b"Sogd"));

pub type hb_user_data_key_t = c_void;

pub type hb_destroy_func_t = extern "C" fn(data: *mut c_void) -> c_void;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct hb_feature_t {
    pub tag: hb_tag_t,
    pub value: u32,
    pub start: c_uint,
    pub end: c_uint,
}

/*impl hb_feature_t {
    #[inline]
    pub fn from_str(str: &str) -> Option<hb_feature_t> {
        let bytes = str.as_bytes();
        let mut feature = unsafe { mem::uninitialized() };
        let r = unsafe {
            hb_feature_from_string(
                bytes.as_ptr() as *const c_char,
                bytes.len() as c_int,
                &mut feature,
            )
        };
        if r != 0 { Some(feature) } else { None }
    }
}*/

extern "C" {
    pub fn hb_feature_from_string(
        str: *const c_char,
        len: c_int,
        feature: *mut hb_feature_t,
    ) -> hb_bool_t;

    pub fn hb_feature_to_string(feature: *mut hb_feature_t, buf: *mut c_char, size: c_uint);
}

#[derive(Clone, Copy/*, Debug*/)]
#[repr(C)]
pub struct hb_variation_t {
    pub tag: hb_tag_t,
    pub value: c_float,
}

/*impl hb_variation_t {
    #[inline]
    pub fn from_str(str: &str) -> Option<hb_variation_t> {
        let bytes = str.as_bytes();
        let mut variation = unsafe { mem::uninitialized() };
        let r = unsafe {
            hb_variation_from_string(
                bytes.as_ptr() as *const c_char,
                bytes.len() as c_int,
                &mut variation,
            )
        };
        if r != 0 { Some(variation) } else { None }
    }
}*/

extern "C" {
    pub fn hb_variation_from_string(
        str: *const c_char,
        len: c_int,
        var: *mut hb_variation_t,
    ) -> hb_bool_t;

    pub fn hb_variation_to_string(var: *mut hb_variation_t, buf: *mut c_char, size: c_uint);
}
