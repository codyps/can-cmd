#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub type __u8 = u8;
pub type __s8 = i8;
pub type __u16 = u16;
pub type __s16 = i16;
pub type __u32 = u32;
pub type __s32 = i32;
pub type __u64 = u64;
pub type __s64 = i64;

include!("bindings.rs");
