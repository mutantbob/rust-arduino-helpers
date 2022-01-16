#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

pub type c_char = i8;
pub type c_schar = i8;
pub type c_uchar = u8;
pub type c_int = i16;
pub type c_uint = u16;
pub type c_ushort = u16;
pub type c_long = i32;
pub type c_ulong = u32;
pub type c_longlong = u32; // not sure if this is true

pub type c_void = core::ffi::c_void;
