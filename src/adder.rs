use ::libc;
#[no_mangle]
pub unsafe extern "C" fn add(mut a: libc::c_int, mut b: libc::c_int)
 -> libc::c_int {
    return a + b;
}
