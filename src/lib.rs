use miniprot_sys::*;

lazy_static::lazy_static! {
    /// Record when this has been initialized. Only has to happen once per run...
    static ref MPSTART: bool = unsafe {
        mp_start();
        true
    };
}

/// Aligner struct, main interface to miniprot
///
/// ```
/// # use miniprot::*;
/// Aligner::builder();
/// ```
#[derive(Clone)]
pub struct Aligner {
    /// Number of threads to create the index with
    pub threads: usize,
    map_options: mp_mapopt_t,
    index: Option<mp_idx_t>,
    reference_file: Option<String>,
    write_index: Option<String>,
    verbosity: u8,
}

impl Default for Aligner {
    fn default() -> Self {
        let mut aligner = Aligner {
            threads: 1,
            map_options: mp_mapopt_t::default(),
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
    // TODO: Should build index as last step... Maybe delay until a mapping function is called?
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
        assert!(verbosity <= 3, "Verbosity must be between 0 and 3");
        self.verbosity = verbosity;
        self
    }

    /// Check that all required options have been set
    pub fn check(&self) {
        assert!(self.reference_file.is_some(), "No reference file specified");
        unsafe {
            assert!(mp_mapopt_check(&self.map_options as *const mp_mapopt_t) == 0, "Invalid map options");
        }
    }

    /// Build the aligner index
    pub fn build(&mut self) {
        self.check();

        let reference = self.reference_file.as_ref().unwrap();
        let reference = std::ffi::CString::new(reference.as_bytes()).unwrap();
        let mut io = mp_idxopt_t::default();
        let mut mi: *mut mp_idx_t = unsafe {
            mp_idx_load(reference.as_ptr() as *const i8, &mut io, self.threads as i32)
        };
        self.index = Some(unsafe { *mi });

        if self.verbosity >= 3 {
            println!("Index built");
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::MaybeUninit;

    #[test]
    fn test_aligner_build() {

        let mut aligner = Aligner::builder()
            .threads(1)
            .with_reference("miniprot-sys/miniprot/test/DPP3-hs.gen.fa.gz")
            .verbosity(3);
        
        aligner.build();

    }
}
