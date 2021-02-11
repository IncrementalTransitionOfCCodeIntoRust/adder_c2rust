#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(main, register_tool)]
use ::adder_c2rust::*;
extern "C" {
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn add(a: libc::c_int, b: libc::c_int) -> libc::c_int;
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut a: libc::c_int = atoi(*argv.offset(1 as libc::c_int as isize));
    let mut b: libc::c_int = atoi(*argv.offset(2 as libc::c_int as isize));
    printf(b"The sum of %d and %d is %d.\n\x00" as *const u8 as
               *const libc::c_char, a, b, add(a, b));
    return 0 as libc::c_int;
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut libc::c_char) as i32)
    }
}
