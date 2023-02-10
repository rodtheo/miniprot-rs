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

impl mp_mapopt_t {
    pub fn no_splice() -> Self {
        unsafe {
            let mut opt = MaybeUninit::uninit();
            mp_mapopt_init(opt.as_mut_ptr());
            let mut opt = opt.assume_init();
            opt.flag = opt.flag | 0x1;
            opt.max_ext = 1000;
            opt.bw = 1000;
            opt.max_intron = 1000;
            opt.io = 10000;
            opt.io_end = 10000;
            opt
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
            let query = std::ffi::CString::new("miniprot/test/DPP3-mm.pep.fa.gz").unwrap();

            let mut mi: *mut mp_idx_t =
                mp_idx_load(reference.as_ptr() as *const i8, io.as_ptr(), 1); // Miniprot initialize index

            let mut mo = mo.assume_init();

            mp_idx_print_stat(mi as *const mp_idx_t, mo.max_occ);

            println!("\n\nMapping results\n\n");
            mp_map_file(
                mi as *const mp_idx_t,
                query.as_ptr() as *const i8,
                &mo,
                1 as i32,
            );

            mp_idx_destroy(mi);
        }
    }
}
