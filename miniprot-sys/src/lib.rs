#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(feature = "bindgen")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(all(not(feature = "bindgen")))]
include!("bindings.rs");


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
}
