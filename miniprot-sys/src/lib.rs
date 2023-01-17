#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(feature = "bindgen")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(all(not(feature = "bindgen")))]
include!("bindings.rs");

use libz_sys::*;

unsafe impl Send for mp_idx_t {}
unsafe impl Send for mp_mapopt_t {}

use std::mem::MaybeUninit;

impl Default for mp_mapopt_t {
    fn default() -> Self {
        unsafe {
            let mut opt = MaybeUninit::uninit();
            mp_mapopt_init(opt.as_mut_ptr());
            opt.assume_init()
        }
    }
}

impl Default for mp_idxopt_t {
    fn default() -> Self {
        unsafe {
            let mut opt = MaybeUninit::uninit();
            mp_idxopt_init(opt.as_mut_ptr());
            opt.assume_init()
        }
    }
}

// TODO: Add more tests!
#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::MaybeUninit;

    #[test]
    fn test_miniprot() {
        unsafe {

            let mut mo = MaybeUninit::uninit();
            let mut io = MaybeUninit::uninit();
            // let mut mi: MaybeUninit<*mut mp_idx_t> = MaybeUninit::uninit();

            mp_start(); // Miniprot initialize some things
            mp_mapopt_init(mo.as_mut_ptr()); // Miniprot initialize map options
            mp_idxopt_init(io.as_mut_ptr()); // Miniprot initialize index options
            mp_mapopt_check(mo.as_ptr()); // Miniprot check map options

            let reference = std::ffi::CString::new("miniprot/test/DPP3-hs.gen.fa.gz").unwrap();

            let mut mi = mp_idx_load(reference.as_ptr() as *const i8, io.as_ptr(), 4); // Miniprot initialize index
        }
    }
}
