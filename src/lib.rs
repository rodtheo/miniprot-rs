use miniprot_sys::*;

use std::cell::RefCell;
use std::mem::MaybeUninit;

lazy_static::lazy_static! {
    /// Record when this has been initialized. Only has to happen once per run...
    static ref MPSTART: bool = unsafe {
        mp_start();
        true
    };
}

thread_local! {
    static BUF: RefCell<ThreadLocalBuffer> = RefCell::new(ThreadLocalBuffer::new());
}

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

// TODO: Aligner should be AlignerBuilder
// and then a finalized build should be Aligner
// TODO: Check that it works with rayon

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
    index_options: mp_idxopt_t,
    index: Option<mp_idx_t>,
    reference_file: Option<String>,
    write_index: Option<String>,
    verbosity: u8,
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
        assert!(*MPSTART, "Miniprot has not been initialized. This should only happen once per run.");

        self.check();

        let reference = self.reference_file.as_ref().unwrap();
        let reference = std::ffi::CString::new(reference.as_bytes()).unwrap();
        let mut mi: *mut mp_idx_t = unsafe {
            mp_idx_load(reference.as_ptr() as *const i8, &self.index_options, self.threads as i32)
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
        // n_regs are the number of results
        let mut n_regs: i32 = 0;
        let mappings = BUF.with(|buf| {
            println!("here 1 - seq len {}", seq.len());

            let name = std::ffi::CString::new(seqname.as_bytes()).unwrap();
            
            let mut mp_reg: MaybeUninit<*mut mp_reg1_t> = MaybeUninit::uninit();

            mp_reg = MaybeUninit::new(unsafe {
                 mp_map(
                    self.index.as_ref().unwrap() as *const mp_idx_t,
                    seq.len() as i32,
                    seq.as_ptr() as *const i8,
                    &mut n_regs,
                    buf.borrow_mut().buf,
                    &self.map_options,
                    name.as_ptr() as *const i8,
                )
            });

            // NO! Can't deref here in case n_regs == 0
            // let result: mp_reg1_t = unsafe { mp_reg.assume_init() };

            let mut mappings = Vec::with_capacity(n_regs as usize);

            // Need to format properly, see:
            // 	ctg = &mi->nt->ctg[r->vid>>1];
            // in format.c for a start (getting the contig name)

            for i in 0..n_regs {
                unsafe {
                    let reg_ptr = (*mp_reg.as_ptr()).offset(i as isize);
                    let const_ptr = reg_ptr as *const mp_reg1_t;
                    let reg: mp_reg1_t = *reg_ptr;

                    let contig: *mut ::std::os::raw::c_char =
                        (*(self.index.unwrap()).seq.offset(reg.rid as isize)).name;

                    let is_primary = reg.parent == reg.id;
                    let alignment = if !reg.p.is_null() {
                        let p = &*reg.p;

                        // calculate the edit distance
                        let nm = reg.blen - reg.mlen + p.n_ambi() as i32;
                        let n_cigar = p.n_cigar;
                        // Create a vector of the cigar blocks
                        let (cigar, cigar_str) = if n_cigar > 0 {
                            let cigar = p
                                .cigar
                                .as_slice(n_cigar as usize)
                                .to_vec()
                                .iter()
                                .map(|c| ((c >> 4) as u32, (c & 0xf) as u8)) // unpack the length and op code
                                .collect::<Vec<(u32, u8)>>();
                            let cigar_str = cigar
                                .iter()
                                .map(|(len, code)| {
                                    let cigar_char = match code {
                                        0 => "M",
                                        1 => "I",
                                        2 => "D",
                                        3 => "N",
                                        4 => "S",
                                        5 => "H",
                                        6 => "P",
                                        7 => "=",
                                        8 => "X",
                                        _ => panic!("Invalid CIGAR code {code}"),
                                    };
                                    format!("{len}{cigar_char}")
                                })
                                .collect::<Vec<String>>()
                                .join("");
                            (Some(cigar), Some(cigar_str))
                        } else {
                            (None, None)
                        };

                        let (cs_str, md_str) = if cs || md {
                            let mut cs_string: *mut libc::c_char = std::ptr::null_mut();
                            let mut m_cs_string: libc::c_int = 0i32;

                            let cs_str = if cs {
                                let _cs_len = mm_gen_cs(
                                    km,
                                    &mut cs_string,
                                    &mut m_cs_string,
                                    &self.idx.unwrap() as *const mm_idx_t,
                                    const_ptr,
                                    seq.as_ptr() as *const i8,
                                    true.into(),
                                );
                                let _cs_string = std::ffi::CStr::from_ptr(cs_string)
                                    .to_str()
                                    .unwrap()
                                    .to_string();
                                Some(_cs_string)
                            } else {
                                None
                            };

                            let md_str = if md {
                                let _md_len = mm_gen_MD(
                                    km,
                                    &mut cs_string,
                                    &mut m_cs_string,
                                    &self.idx.unwrap() as *const mm_idx_t,
                                    const_ptr,
                                    seq.as_ptr() as *const i8,
                                );
                                let _md_string = std::ffi::CStr::from_ptr(cs_string)
                                    .to_str()
                                    .unwrap()
                                    .to_string();
                                Some(_md_string)
                            } else {
                                None
                            };
                            (cs_str, md_str)
                        } else {
                            (None, None)
                        };

                        Some(Alignment {
                            nm,
                            cigar,
                            cigar_str,
                            md: md_str,
                            cs: cs_str,
                        })
                    } else {
                        None
                    };
                    mappings.push(Mapping {
                        target_name: Some(
                            std::ffi::CStr::from_ptr(contig)
                                .to_str()
                                .unwrap()
                                .to_string(),
                        ),
                        target_len: (*(self.idx.unwrap()).seq.offset(reg.rid as isize)).len as i32,
                        target_start: reg.rs,
                        target_end: reg.re,
                        query_name: None,
                        query_len: NonZeroI32::new(seq.len() as i32),
                        query_start: reg.qs,
                        query_end: reg.qe,
                        strand: if reg.rev() == 0 {
                            Strand::Forward
                        } else {
                            Strand::Reverse
                        },
                        match_len: reg.mlen,
                        block_len: reg.blen,
                        mapq: reg.mapq(),
                        is_primary,
                        alignment,
                    });
                }
            }

            mappings
        });

        Ok(mappings)

        // println!("Mappings: {}", mappings);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::MaybeUninit;
    use fffx::Fasta;

    #[test]
    fn test_aligner_build() {

        let mut aligner = Aligner::builder()
            .threads(1)
            .with_reference("miniprot-sys/miniprot/test/DPP3-hs.gen.fa.gz")
            .verbosity(3);
        
        aligner.build();
    }

    #[test]
    fn test_aligner_mapping() {
        let mut aligner = Aligner::builder()
            .threads(1)
            .with_reference("miniprot-sys/miniprot/test/DPP3-hs.gen.fa.gz")
            .verbosity(3);
        
        aligner.build();

        let seqs = vec!["MAGIIKKQILKHLSRFTKNLSPDKINLSTLKGEGELKNLELDEEVLQNMLDLPTWLAINKVFCNKASIRIPWTKLKTHPICLSLDKVIMEMSTCEEPRSPNGP",
                        "SPIATASGQSEYGFAEKVVEGISVSVNSIVIRIGAKAFNASFELSQLRIYSVNAHWEHGDLRFTRIQDPQRGEVLTFKEINWQMIRIEADATQSSHLEIM",
                        "CAPVRLITNQSKIRVTLKRRLKDCNVIATKLVLILDDLLWVLTDSQLKAMVQYAKSLSEAIEKSTEQRKSMAPEPTQSSTVVASAQQVKTTQTSNAPDVNDAIVKLFNDFDVKETSHHLVISHLDLHICDDIHAKEKESNRRITGGAMQLSFTQLTIDYYPYHKAGDSCNHWMYFSDATKTKNGWANELLHEFECNVEMLKQAVKDHNVGSPPKSPTHASPQHTQTEKDYPLKGTCRTPSVLSQQSKAKLMSSSVVVRLADFNIYQVSTAEQCRSSPKSMICCNKKSLYLPQEMSAVYIEFTEYYYPDGKDFPIPSPNLYSQLNALQFTVDERSILWLNQFLLDLKQSLNQFMAVYKLNDNSKSDEHVDVRVDGLMLKFVIPSEVKSECHQDQPRAISIQSSEMIATNTRHCPNCRHSDLEALFQDFKDCDFFSKTYTSFPKSCDNFNLLHPIFQRHAHEQDTKMHEIYKGNITPQLNKNTLKTSAATDVWAVYFSQFWIDYEGMKSGKGRPISFVDSFPLSIWICQPTRYAESQKEPQTCNQVSLNTSQSESSDLAGRLKRKKLLKEYYSTESEPLTNGGQKPSSSDTFFRFSPSSSEADIHLLVHVHKHVSMQINHYQYLLLLFLHESLILLSENLRKDVEAVTGSPASQTSICIGILLRSAELALLLHPVDQANTLKSPV",
                        "SESVSPVVPDYLPTENGDFLSSKRKQISRDINRIRSVTVNHMSDNRSMSVDLSHIPLKDPLLFKSASDTNLQKGISFMDYLSDKHLGKISEDESSGLVYK",
                        "SGSGEIGSETSDKKDSFYTDSSSILNYREDSNILSFDSDGNQNILSSTLTSKGNETIESIFKAEDLLPEAASLSENLDISKEETPPVRTLKSQSSLSGKPKERCPPNLAPLCVSYKNMKRSS",
                        "SQMSLDTISLDSMILEEQLLESDGSDSHMFLEKGNKKNSTTNYRGTAESVNAGANLQNYGETSPDAISTNSEGAQENHDDLMSVVVFKITGVNGEIDIRGEDTEICLQVNQVTPDQLGNISL",
                        "RHYLCNRPVGSDQKAVIHSKSSPEISLRFESGPGAVIHSLLAEKNGFLQCHIENFSTEFLTSSLMNIQHFLEDETVATVMPMKIQVSNTKINLKDDSPRSSTVSLEPAPVTVHIDHLVV",
                        "ERSDDGSFHIRDSHMLNTGNDLKENVKSDSVLLTSGKYDLKKQRSVTQATQTSPGVPWPSQSANFPEFSFDFTREQLMEENESLKQELAKAKMALAEAHLEKDALLHHIKKMTVE",
                        ];

        for seq in seqs {
            aligner.map("test", seq.as_bytes());
        }

        let working_seq = "MADTQYILPNDIGVSSLDCREAFRLLSPTERLYAHHLSRAAWYGGLAVLLQTSPEAPYIYALLSRLFRAQDPDQLRQHALAEGLTEEEYQAFLVYAAGVYSNMGNYKSFGDTKFVPNLPKDKLGRVILGSKAAQQRPEEVRDLWQTCGDLMFSLEPRLRHLGLGKEGVTTYFSGDCTMEDAKLAQDFLDSQNLSAYNTRLFKVVGQEGKSHYEVRLASVLNTDPALDSELTSKLKRYEFQGNHFQVTRGDYAPILQKVVEHLEKAKAYAANSHQEQMLAQYVESFTQGSIEAHKRGSRFWIQDKGPIVESYIGFIESYRDPFGSRGEFEGFVAMVNKAMSAKFERLVASAEQLLKELPWPLAFEKDKFLTPDFTSLDVLTFAGSGIPAGINIPNYDDLRQTEGFKNVSLGNVLAVAYAAKREKLTFLEEEDKDLYIRWKGPSFDVQVGLHELLGHGSGKLFVQDEKGAFNFDKETVINPETGEQIQSWYRSGETWDSKFSTIASSYEECRAESVGLYLCLNPQVLEIFGFEGADAEDVIYVNWLNMVRAGLLALEFYTPEAANWRQAHMQARFVILRVLLEAGEGLVTVTPTTGSDGRPDARVRLDRSKIRSVGRPALERFLRRLQVLKSTGDVVAGRALYEGYAAVTDAPPECFLTLRDTVLLRKESRKLIVQPNTRLEGSEVQLVEYEASAAGLIRSFCERFPEDGPELEEVLIQLAAADARFWRNQAQEAPPGQA";
        aligner.map("working seq", working_seq.as_bytes());

        let working_seq = "DPDQLRQHALAEGLTEEEYQAFLVYAAGVYSNMGNYKSFGDTKFVPNLPKDKL";
        aligner.map("working seq2", working_seq.as_bytes());
    }

        

}
