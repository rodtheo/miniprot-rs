#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use miniprot_sys::*;
use noodles::gff::record::Strand;
use std::cell::RefCell;
use std::mem::MaybeUninit;
use std::string::ToString;
use strum_macros::Display;
use strum_macros::FromRepr;
pub enum MiniprotAlignmentOperation {
    #[strum(serialize = "M")]
    Match(usize),
    #[strum(serialize = "I")]
    Ins(usize),
    #[strum(serialize = "D")]
    Del(usize),
    #[strum(serialize = "N")]
    N(usize),
    #[strum(serialize = "S")]
    Subst(usize),
    #[strum(serialize = "H")]
    H(usize),
    #[strum(serialize = "P")]
    P(usize),
    #[strum(serialize = "=")]
    Equal(usize),
    #[strum(serialize = "X")]
    X(usize),
    #[strum(serialize = "B")]
    B(usize),
    #[strum(serialize = "F")]
    F(usize),
    #[strum(serialize = "G")]
    G(usize),
    #[strum(serialize = "U")]
    U(usize),
    #[strum(serialize = "V")]
    V(usize),
    #[strum(serialize = "E")]
    E(usize),
    #[strum(serialize = "Warning")]
    Warning,
}
#[allow(clippy::use_self)]
impl MiniprotAlignmentOperation {
    ///Try to create [Self] from the raw representation
    pub fn from_repr(discriminant: usize) -> Option<MiniprotAlignmentOperation> {
        const MATCH_DISCRIMINANT: usize = 0;
        const INS_DISCRIMINANT: usize = MATCH_DISCRIMINANT + 1;
        const DEL_DISCRIMINANT: usize = INS_DISCRIMINANT + 1;
        const N_DISCRIMINANT: usize = DEL_DISCRIMINANT + 1;
        const SUBST_DISCRIMINANT: usize = N_DISCRIMINANT + 1;
        const H_DISCRIMINANT: usize = SUBST_DISCRIMINANT + 1;
        const P_DISCRIMINANT: usize = H_DISCRIMINANT + 1;
        const EQUAL_DISCRIMINANT: usize = P_DISCRIMINANT + 1;
        const X_DISCRIMINANT: usize = EQUAL_DISCRIMINANT + 1;
        const B_DISCRIMINANT: usize = X_DISCRIMINANT + 1;
        const F_DISCRIMINANT: usize = B_DISCRIMINANT + 1;
        const G_DISCRIMINANT: usize = F_DISCRIMINANT + 1;
        const U_DISCRIMINANT: usize = G_DISCRIMINANT + 1;
        const V_DISCRIMINANT: usize = U_DISCRIMINANT + 1;
        const E_DISCRIMINANT: usize = V_DISCRIMINANT + 1;
        const WARNING_DISCRIMINANT: usize = E_DISCRIMINANT + 1;
        match discriminant {
            v if v == MATCH_DISCRIMINANT => ::core::option::Option::Some(
                MiniprotAlignmentOperation::Match(::core::default::Default::default()),
            ),
            v if v == INS_DISCRIMINANT => ::core::option::Option::Some(
                MiniprotAlignmentOperation::Ins(::core::default::Default::default()),
            ),
            v if v == DEL_DISCRIMINANT => ::core::option::Option::Some(
                MiniprotAlignmentOperation::Del(::core::default::Default::default()),
            ),
            v if v == N_DISCRIMINANT => ::core::option::Option::Some(
                MiniprotAlignmentOperation::N(::core::default::Default::default()),
            ),
            v if v == SUBST_DISCRIMINANT => ::core::option::Option::Some(
                MiniprotAlignmentOperation::Subst(::core::default::Default::default()),
            ),
            v if v == H_DISCRIMINANT => ::core::option::Option::Some(
                MiniprotAlignmentOperation::H(::core::default::Default::default()),
            ),
            v if v == P_DISCRIMINANT => ::core::option::Option::Some(
                MiniprotAlignmentOperation::P(::core::default::Default::default()),
            ),
            v if v == EQUAL_DISCRIMINANT => ::core::option::Option::Some(
                MiniprotAlignmentOperation::Equal(::core::default::Default::default()),
            ),
            v if v == X_DISCRIMINANT => ::core::option::Option::Some(
                MiniprotAlignmentOperation::X(::core::default::Default::default()),
            ),
            v if v == B_DISCRIMINANT => ::core::option::Option::Some(
                MiniprotAlignmentOperation::B(::core::default::Default::default()),
            ),
            v if v == F_DISCRIMINANT => ::core::option::Option::Some(
                MiniprotAlignmentOperation::F(::core::default::Default::default()),
            ),
            v if v == G_DISCRIMINANT => ::core::option::Option::Some(
                MiniprotAlignmentOperation::G(::core::default::Default::default()),
            ),
            v if v == U_DISCRIMINANT => ::core::option::Option::Some(
                MiniprotAlignmentOperation::U(::core::default::Default::default()),
            ),
            v if v == V_DISCRIMINANT => ::core::option::Option::Some(
                MiniprotAlignmentOperation::V(::core::default::Default::default()),
            ),
            v if v == E_DISCRIMINANT => ::core::option::Option::Some(
                MiniprotAlignmentOperation::E(::core::default::Default::default()),
            ),
            v if v == WARNING_DISCRIMINANT => {
                ::core::option::Option::Some(MiniprotAlignmentOperation::Warning)
            }
            _ => ::core::option::Option::None,
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for MiniprotAlignmentOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match (&*self,) {
            (&MiniprotAlignmentOperation::Match(ref __self_0),) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Match");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&MiniprotAlignmentOperation::Ins(ref __self_0),) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Ins");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&MiniprotAlignmentOperation::Del(ref __self_0),) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Del");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&MiniprotAlignmentOperation::N(ref __self_0),) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "N");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&MiniprotAlignmentOperation::Subst(ref __self_0),) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Subst");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&MiniprotAlignmentOperation::H(ref __self_0),) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "H");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&MiniprotAlignmentOperation::P(ref __self_0),) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "P");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&MiniprotAlignmentOperation::Equal(ref __self_0),) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Equal");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&MiniprotAlignmentOperation::X(ref __self_0),) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "X");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&MiniprotAlignmentOperation::B(ref __self_0),) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "B");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&MiniprotAlignmentOperation::F(ref __self_0),) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "F");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&MiniprotAlignmentOperation::G(ref __self_0),) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "G");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&MiniprotAlignmentOperation::U(ref __self_0),) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "U");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&MiniprotAlignmentOperation::V(ref __self_0),) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "V");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&MiniprotAlignmentOperation::E(ref __self_0),) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "E");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&MiniprotAlignmentOperation::Warning,) => {
                ::core::fmt::Formatter::write_str(f, "Warning")
            }
        }
    }
}
impl ::core::fmt::Display for MiniprotAlignmentOperation {
    fn fmt(
        &self,
        f: &mut ::core::fmt::Formatter,
    ) -> ::core::result::Result<(), ::core::fmt::Error> {
        match *self {
            MiniprotAlignmentOperation::Match(..) => f.pad("M"),
            MiniprotAlignmentOperation::Ins(..) => f.pad("I"),
            MiniprotAlignmentOperation::Del(..) => f.pad("D"),
            MiniprotAlignmentOperation::N(..) => f.pad("N"),
            MiniprotAlignmentOperation::Subst(..) => f.pad("S"),
            MiniprotAlignmentOperation::H(..) => f.pad("H"),
            MiniprotAlignmentOperation::P(..) => f.pad("P"),
            MiniprotAlignmentOperation::Equal(..) => f.pad("="),
            MiniprotAlignmentOperation::X(..) => f.pad("X"),
            MiniprotAlignmentOperation::B(..) => f.pad("B"),
            MiniprotAlignmentOperation::F(..) => f.pad("F"),
            MiniprotAlignmentOperation::G(..) => f.pad("G"),
            MiniprotAlignmentOperation::U(..) => f.pad("U"),
            MiniprotAlignmentOperation::V(..) => f.pad("V"),
            MiniprotAlignmentOperation::E(..) => f.pad("E"),
            MiniprotAlignmentOperation::Warning => f.pad("Warning"),
        }
    }
}
impl ::core::marker::StructuralPartialEq for MiniprotAlignmentOperation {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for MiniprotAlignmentOperation {
    #[inline]
    fn eq(&self, other: &MiniprotAlignmentOperation) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (
                        &MiniprotAlignmentOperation::Match(ref __self_0),
                        &MiniprotAlignmentOperation::Match(ref __arg_1_0),
                    ) => (*__self_0) == (*__arg_1_0),
                    (
                        &MiniprotAlignmentOperation::Ins(ref __self_0),
                        &MiniprotAlignmentOperation::Ins(ref __arg_1_0),
                    ) => (*__self_0) == (*__arg_1_0),
                    (
                        &MiniprotAlignmentOperation::Del(ref __self_0),
                        &MiniprotAlignmentOperation::Del(ref __arg_1_0),
                    ) => (*__self_0) == (*__arg_1_0),
                    (
                        &MiniprotAlignmentOperation::N(ref __self_0),
                        &MiniprotAlignmentOperation::N(ref __arg_1_0),
                    ) => (*__self_0) == (*__arg_1_0),
                    (
                        &MiniprotAlignmentOperation::Subst(ref __self_0),
                        &MiniprotAlignmentOperation::Subst(ref __arg_1_0),
                    ) => (*__self_0) == (*__arg_1_0),
                    (
                        &MiniprotAlignmentOperation::H(ref __self_0),
                        &MiniprotAlignmentOperation::H(ref __arg_1_0),
                    ) => (*__self_0) == (*__arg_1_0),
                    (
                        &MiniprotAlignmentOperation::P(ref __self_0),
                        &MiniprotAlignmentOperation::P(ref __arg_1_0),
                    ) => (*__self_0) == (*__arg_1_0),
                    (
                        &MiniprotAlignmentOperation::Equal(ref __self_0),
                        &MiniprotAlignmentOperation::Equal(ref __arg_1_0),
                    ) => (*__self_0) == (*__arg_1_0),
                    (
                        &MiniprotAlignmentOperation::X(ref __self_0),
                        &MiniprotAlignmentOperation::X(ref __arg_1_0),
                    ) => (*__self_0) == (*__arg_1_0),
                    (
                        &MiniprotAlignmentOperation::B(ref __self_0),
                        &MiniprotAlignmentOperation::B(ref __arg_1_0),
                    ) => (*__self_0) == (*__arg_1_0),
                    (
                        &MiniprotAlignmentOperation::F(ref __self_0),
                        &MiniprotAlignmentOperation::F(ref __arg_1_0),
                    ) => (*__self_0) == (*__arg_1_0),
                    (
                        &MiniprotAlignmentOperation::G(ref __self_0),
                        &MiniprotAlignmentOperation::G(ref __arg_1_0),
                    ) => (*__self_0) == (*__arg_1_0),
                    (
                        &MiniprotAlignmentOperation::U(ref __self_0),
                        &MiniprotAlignmentOperation::U(ref __arg_1_0),
                    ) => (*__self_0) == (*__arg_1_0),
                    (
                        &MiniprotAlignmentOperation::V(ref __self_0),
                        &MiniprotAlignmentOperation::V(ref __arg_1_0),
                    ) => (*__self_0) == (*__arg_1_0),
                    (
                        &MiniprotAlignmentOperation::E(ref __self_0),
                        &MiniprotAlignmentOperation::E(ref __arg_1_0),
                    ) => (*__self_0) == (*__arg_1_0),
                    _ => true,
                }
            } else {
                false
            }
        }
    }
    #[inline]
    fn ne(&self, other: &MiniprotAlignmentOperation) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (
                        &MiniprotAlignmentOperation::Match(ref __self_0),
                        &MiniprotAlignmentOperation::Match(ref __arg_1_0),
                    ) => (*__self_0) != (*__arg_1_0),
                    (
                        &MiniprotAlignmentOperation::Ins(ref __self_0),
                        &MiniprotAlignmentOperation::Ins(ref __arg_1_0),
                    ) => (*__self_0) != (*__arg_1_0),
                    (
                        &MiniprotAlignmentOperation::Del(ref __self_0),
                        &MiniprotAlignmentOperation::Del(ref __arg_1_0),
                    ) => (*__self_0) != (*__arg_1_0),
                    (
                        &MiniprotAlignmentOperation::N(ref __self_0),
                        &MiniprotAlignmentOperation::N(ref __arg_1_0),
                    ) => (*__self_0) != (*__arg_1_0),
                    (
                        &MiniprotAlignmentOperation::Subst(ref __self_0),
                        &MiniprotAlignmentOperation::Subst(ref __arg_1_0),
                    ) => (*__self_0) != (*__arg_1_0),
                    (
                        &MiniprotAlignmentOperation::H(ref __self_0),
                        &MiniprotAlignmentOperation::H(ref __arg_1_0),
                    ) => (*__self_0) != (*__arg_1_0),
                    (
                        &MiniprotAlignmentOperation::P(ref __self_0),
                        &MiniprotAlignmentOperation::P(ref __arg_1_0),
                    ) => (*__self_0) != (*__arg_1_0),
                    (
                        &MiniprotAlignmentOperation::Equal(ref __self_0),
                        &MiniprotAlignmentOperation::Equal(ref __arg_1_0),
                    ) => (*__self_0) != (*__arg_1_0),
                    (
                        &MiniprotAlignmentOperation::X(ref __self_0),
                        &MiniprotAlignmentOperation::X(ref __arg_1_0),
                    ) => (*__self_0) != (*__arg_1_0),
                    (
                        &MiniprotAlignmentOperation::B(ref __self_0),
                        &MiniprotAlignmentOperation::B(ref __arg_1_0),
                    ) => (*__self_0) != (*__arg_1_0),
                    (
                        &MiniprotAlignmentOperation::F(ref __self_0),
                        &MiniprotAlignmentOperation::F(ref __arg_1_0),
                    ) => (*__self_0) != (*__arg_1_0),
                    (
                        &MiniprotAlignmentOperation::G(ref __self_0),
                        &MiniprotAlignmentOperation::G(ref __arg_1_0),
                    ) => (*__self_0) != (*__arg_1_0),
                    (
                        &MiniprotAlignmentOperation::U(ref __self_0),
                        &MiniprotAlignmentOperation::U(ref __arg_1_0),
                    ) => (*__self_0) != (*__arg_1_0),
                    (
                        &MiniprotAlignmentOperation::V(ref __self_0),
                        &MiniprotAlignmentOperation::V(ref __arg_1_0),
                    ) => (*__self_0) != (*__arg_1_0),
                    (
                        &MiniprotAlignmentOperation::E(ref __self_0),
                        &MiniprotAlignmentOperation::E(ref __arg_1_0),
                    ) => (*__self_0) != (*__arg_1_0),
                    _ => false,
                }
            } else {
                true
            }
        }
    }
}
impl Default for MiniprotAlignmentOperation {
    fn default() -> Self {
        Self::Warning
    }
}
#[allow(missing_copy_implementations)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
/// Record when this has been initialized. Only has to happen once per run...
struct MPSTART {
    __private_field: (),
}
#[doc(hidden)]
static MPSTART: MPSTART = MPSTART {
    __private_field: (),
};
impl ::lazy_static::__Deref for MPSTART {
    type Target = bool;
    fn deref(&self) -> &bool {
        #[inline(always)]
        fn __static_ref_initialize() -> bool {
            unsafe {
                mp_start();
                true
            }
        }
        #[inline(always)]
        fn __stability() -> &'static bool {
            static LAZY: ::lazy_static::lazy::Lazy<bool> = ::lazy_static::lazy::Lazy::INIT;
            LAZY.get(__static_ref_initialize)
        }
        __stability()
    }
}
impl ::lazy_static::LazyStatic for MPSTART {
    fn initialize(lazy: &Self) {
        let _ = &**lazy;
    }
}
const BUF: ::std::thread::LocalKey<RefCell<ThreadLocalBuffer>> = {
    #[inline]
    fn __init() -> RefCell<ThreadLocalBuffer> {
        RefCell::new(ThreadLocalBuffer::new())
    }
    #[inline]
    unsafe fn __getit() -> ::std::option::Option<&'static RefCell<ThreadLocalBuffer>> {
        #[thread_local]
        #[cfg(all(
            target_thread_local,
            not(all(target_family = "wasm", not(target_feature = "atomics"))),
        ))]
        static __KEY: ::std::thread::__FastLocalKeyInner<RefCell<ThreadLocalBuffer>> =
            ::std::thread::__FastLocalKeyInner::new();
        #[allow(unused_unsafe)]
        unsafe {
            __KEY.get(__init)
        }
    }
    unsafe { ::std::thread::LocalKey::new(__getit) }
};
/// ThreadLocalBuffer for minimap2 memory management
struct ThreadLocalBuffer {
    buf: *mut mp_tbuf_t,
}
impl ThreadLocalBuffer {
    pub fn new() -> Self {
        let buf = unsafe { mp_tbuf_init() };
        Self { buf }
    }
}
/// Handle destruction of thread local buffer properly.
impl Drop for ThreadLocalBuffer {
    fn drop(&mut self) {
        unsafe { mp_tbuf_destroy(self.buf) };
    }
}
impl Default for ThreadLocalBuffer {
    fn default() -> Self {
        Self::new()
    }
}
/// Aligner struct, main interface to miniprot
///
/// ```
/// # use miniprot::*;
/// Aligner::builder();
/// ```
pub struct Aligner {
    /// Number of threads to create the index with
    pub threads: usize,
    map_options: mp_mapopt_t,
    index_options: mp_idxopt_t,
    index: Option<mp_idx_t>,
    reference_file: Option<String>,
    write_index: Option<String>,
    verbosity: u8,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Aligner {
    #[inline]
    fn clone(&self) -> Aligner {
        match *self {
            Aligner {
                threads: ref __self_0_0,
                map_options: ref __self_0_1,
                index_options: ref __self_0_2,
                index: ref __self_0_3,
                reference_file: ref __self_0_4,
                write_index: ref __self_0_5,
                verbosity: ref __self_0_6,
            } => Aligner {
                threads: ::core::clone::Clone::clone(&(*__self_0_0)),
                map_options: ::core::clone::Clone::clone(&(*__self_0_1)),
                index_options: ::core::clone::Clone::clone(&(*__self_0_2)),
                index: ::core::clone::Clone::clone(&(*__self_0_3)),
                reference_file: ::core::clone::Clone::clone(&(*__self_0_4)),
                write_index: ::core::clone::Clone::clone(&(*__self_0_5)),
                verbosity: ::core::clone::Clone::clone(&(*__self_0_6)),
            },
        }
    }
}
impl Default for Aligner {
    fn default() -> Self {
        let aligner = Aligner {
            threads: 1,
            map_options: mp_mapopt_t::default(),
            index_options: mp_idxopt_t::default(),
            index: None,
            reference_file: None,
            write_index: None,
            verbosity: 0,
        };
        aligner
    }
}
impl Aligner {
    /// Builder convenience function
    pub fn builder() -> Self {
        Self::default()
    }
    /// Load an index, part of builder pattern
    /// reference argument should either be a filename to a FASTA file or a previously built
    /// and saved index.
    /// This immediately builds the index, so you need to have options and n_threads set before calling...
    pub fn with_reference(mut self, reference: &str) -> Self {
        self.reference_file = Some(reference.to_string());
        self
    }
    /// Set the number of threads
    pub fn threads(mut self, threads: usize) -> Self {
        self.threads = threads;
        self
    }
    /// Save the resulting index to a file
    pub fn write_index(mut self, output_file: &str) -> Self {
        self.write_index = Some(output_file.to_string());
        self
    }
    /// Set the verbosity level
    pub fn verbosity(mut self, verbosity: u8) -> Self {
        if !(verbosity <= 3) {
            ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                &["Verbosity must be between 0 and 3"],
                &match () {
                    _args => [],
                },
            ))
        };
        self.verbosity = verbosity;
        self
    }
    /// Check that all required options have been set
    pub fn check(&self) {
        if !self.reference_file.is_some() {
            ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                &["No reference file specified"],
                &match () {
                    _args => [],
                },
            ))
        };
        unsafe {
            if !(mp_mapopt_check(&self.map_options as *const mp_mapopt_t) == 0) {
                ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                    &["Invalid map options"],
                    &match () {
                        _args => [],
                    },
                ))
            };
        }
    }
    /// Build the aligner index
    pub fn build(&mut self) {
        if !*MPSTART {
            ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                &["Miniprot has not been initialized. This should only happen once per run."],
                &match () {
                    _args => [],
                },
            ))
        };
        self.check();
        let reference = self.reference_file.as_ref().unwrap();
        let reference = std::ffi::CString::new(reference.as_bytes()).unwrap();
        let mut mi: *mut mp_idx_t = unsafe {
            mp_idx_load(
                reference.as_ptr() as *const i8,
                &self.index_options,
                self.threads as i32,
            )
        };
        self.index = Some(unsafe { *mi });
        if self.verbosity >= 3 {
            unsafe {
                mp_idx_print_stat(mi, self.map_options.max_occ as i32);
            }
        }
        if let Some(output_file) = &self.write_index {
            let output_file = std::ffi::CString::new(output_file.as_bytes()).unwrap();
            unsafe {
                mp_idx_dump(output_file.as_ptr() as *const i8, mi);
            }
        }
    }
    /// Map a sequence to the index
    pub fn map(&self, seqname: &str, seq: &[u8]) {
        let mut n_regs: i32 = 0;
        let buf = ThreadLocalBuffer::new();
        {
            ::std::io::_print(::core::fmt::Arguments::new_v1(
                &["here 1 - seq len ", "\n"],
                &match (&seq.len(),) {
                    _args => [::core::fmt::ArgumentV1::new(
                        _args.0,
                        ::core::fmt::Display::fmt,
                    )],
                },
            ));
        };
        let name = std::ffi::CString::new(seqname.as_bytes()).unwrap();
        let mut mp_reg: MaybeUninit<*mut mp_reg1_t> = MaybeUninit::uninit();
        mp_reg = MaybeUninit::new(unsafe {
            mp_map(
                self.index.as_ref().unwrap() as *const mp_idx_t,
                seq.len() as i32,
                seq.as_ptr() as *const i8,
                &mut n_regs,
                buf.buf,
                &self.map_options,
                name.as_ptr() as *const i8,
            )
        });
        for i in 0..n_regs {
            unsafe {
                let reg_ptr = (*mp_reg.as_ptr()).offset(i as isize);
                let const_ptr = reg_ptr as *const mp_reg1_t;
                let reg: mp_reg1_t = *reg_ptr;
                let is_primary = reg.parent == reg.id;
                let alignment = if !reg.p.is_null() {
                    let p = &*reg.p;
                    let n_cigar = p.n_cigar;
                    let (cigar, cigar_str) = if n_cigar > 0 {
                        let cigar = p
                            .cigar
                            .as_slice(n_cigar as usize)
                            .to_vec()
                            .iter()
                            .map(|c| {
                                (
                                    (c >> 4) as u32,
                                    MiniprotAlignmentOperation::from_repr((c & 0xf) as usize)
                                        .unwrap(),
                                )
                            })
                            .collect::<Vec<(u32, MiniprotAlignmentOperation)>>();
                        let cigar_str = cigar
                            .iter()
                            .map(|(len, code)| {
                                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                    &["", ""],
                                    &match (&len, &code) {
                                        _args => [
                                            ::core::fmt::ArgumentV1::new(
                                                _args.0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                _args.1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ));
                                res
                            })
                            .collect::<Vec<String>>()
                            .join("");
                        (Some(cigar), Some(cigar_str))
                    } else {
                        (None, None)
                    };
                    match cigar {
                        tmp => {
                            {
                                ::std::io::_eprint(::core::fmt::Arguments::new_v1_formatted(
                                    &["[", ":", "] ", " = ", "\n"],
                                    &match (&"src/lib.rs", &306u32, &"cigar", &&tmp) {
                                        _args => [
                                            ::core::fmt::ArgumentV1::new(
                                                _args.0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                _args.1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                _args.2,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                _args.3,
                                                ::core::fmt::Debug::fmt,
                                            ),
                                        ],
                                    },
                                    &[
                                        ::core::fmt::rt::v1::Argument {
                                            position: 0usize,
                                            format: ::core::fmt::rt::v1::FormatSpec {
                                                fill: ' ',
                                                align: ::core::fmt::rt::v1::Alignment::Unknown,
                                                flags: 0u32,
                                                precision: ::core::fmt::rt::v1::Count::Implied,
                                                width: ::core::fmt::rt::v1::Count::Implied,
                                            },
                                        },
                                        ::core::fmt::rt::v1::Argument {
                                            position: 1usize,
                                            format: ::core::fmt::rt::v1::FormatSpec {
                                                fill: ' ',
                                                align: ::core::fmt::rt::v1::Alignment::Unknown,
                                                flags: 0u32,
                                                precision: ::core::fmt::rt::v1::Count::Implied,
                                                width: ::core::fmt::rt::v1::Count::Implied,
                                            },
                                        },
                                        ::core::fmt::rt::v1::Argument {
                                            position: 2usize,
                                            format: ::core::fmt::rt::v1::FormatSpec {
                                                fill: ' ',
                                                align: ::core::fmt::rt::v1::Alignment::Unknown,
                                                flags: 0u32,
                                                precision: ::core::fmt::rt::v1::Count::Implied,
                                                width: ::core::fmt::rt::v1::Count::Implied,
                                            },
                                        },
                                        ::core::fmt::rt::v1::Argument {
                                            position: 3usize,
                                            format: ::core::fmt::rt::v1::FormatSpec {
                                                fill: ' ',
                                                align: ::core::fmt::rt::v1::Alignment::Unknown,
                                                flags: 4u32,
                                                precision: ::core::fmt::rt::v1::Count::Implied,
                                                width: ::core::fmt::rt::v1::Count::Implied,
                                            },
                                        },
                                    ],
                                    unsafe { ::core::fmt::UnsafeArg::new() },
                                ));
                            };
                            tmp
                        }
                    };
                    match cigar_str {
                        tmp => {
                            {
                                ::std::io::_eprint(::core::fmt::Arguments::new_v1_formatted(
                                    &["[", ":", "] ", " = ", "\n"],
                                    &match (&"src/lib.rs", &307u32, &"cigar_str", &&tmp) {
                                        _args => [
                                            ::core::fmt::ArgumentV1::new(
                                                _args.0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                _args.1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                _args.2,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                _args.3,
                                                ::core::fmt::Debug::fmt,
                                            ),
                                        ],
                                    },
                                    &[
                                        ::core::fmt::rt::v1::Argument {
                                            position: 0usize,
                                            format: ::core::fmt::rt::v1::FormatSpec {
                                                fill: ' ',
                                                align: ::core::fmt::rt::v1::Alignment::Unknown,
                                                flags: 0u32,
                                                precision: ::core::fmt::rt::v1::Count::Implied,
                                                width: ::core::fmt::rt::v1::Count::Implied,
                                            },
                                        },
                                        ::core::fmt::rt::v1::Argument {
                                            position: 1usize,
                                            format: ::core::fmt::rt::v1::FormatSpec {
                                                fill: ' ',
                                                align: ::core::fmt::rt::v1::Alignment::Unknown,
                                                flags: 0u32,
                                                precision: ::core::fmt::rt::v1::Count::Implied,
                                                width: ::core::fmt::rt::v1::Count::Implied,
                                            },
                                        },
                                        ::core::fmt::rt::v1::Argument {
                                            position: 2usize,
                                            format: ::core::fmt::rt::v1::FormatSpec {
                                                fill: ' ',
                                                align: ::core::fmt::rt::v1::Alignment::Unknown,
                                                flags: 0u32,
                                                precision: ::core::fmt::rt::v1::Count::Implied,
                                                width: ::core::fmt::rt::v1::Count::Implied,
                                            },
                                        },
                                        ::core::fmt::rt::v1::Argument {
                                            position: 3usize,
                                            format: ::core::fmt::rt::v1::FormatSpec {
                                                fill: ' ',
                                                align: ::core::fmt::rt::v1::Alignment::Unknown,
                                                flags: 4u32,
                                                precision: ::core::fmt::rt::v1::Count::Implied,
                                                width: ::core::fmt::rt::v1::Count::Implied,
                                            },
                                        },
                                    ],
                                    unsafe { ::core::fmt::UnsafeArg::new() },
                                ));
                            };
                            tmp
                        }
                    };
                };
            }
        }
    }
}
