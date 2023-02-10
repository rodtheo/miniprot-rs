use miniprot_sys::*;
use noodles::gff::record::Strand;

use std::cell::RefCell;
use std::mem::MaybeUninit;

use std::string::ToString;
use strum_macros::Display;
use strum_macros::FromRepr;

#[derive(Debug, Display, PartialEq)]
pub enum MiniprotAlignmentOperation {
    // MIDNSHP=XBFGUVE
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
    pub fn from_repr(discriminant: usize, att: usize) -> Option<MiniprotAlignmentOperation> {
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
            v if v == MATCH_DISCRIMINANT => {
                ::core::option::Option::Some(MiniprotAlignmentOperation::Match(att))
            }
            v if v == INS_DISCRIMINANT => {
                ::core::option::Option::Some(MiniprotAlignmentOperation::Ins(att))
            }
            v if v == DEL_DISCRIMINANT => {
                ::core::option::Option::Some(MiniprotAlignmentOperation::Del(att))
            }
            v if v == N_DISCRIMINANT => {
                ::core::option::Option::Some(MiniprotAlignmentOperation::N(att))
            }
            v if v == SUBST_DISCRIMINANT => {
                ::core::option::Option::Some(MiniprotAlignmentOperation::Subst(att))
            }
            v if v == H_DISCRIMINANT => {
                ::core::option::Option::Some(MiniprotAlignmentOperation::H(att))
            }
            v if v == P_DISCRIMINANT => {
                ::core::option::Option::Some(MiniprotAlignmentOperation::P(att))
            }
            v if v == EQUAL_DISCRIMINANT => {
                ::core::option::Option::Some(MiniprotAlignmentOperation::Equal(att))
            }
            v if v == X_DISCRIMINANT => {
                ::core::option::Option::Some(MiniprotAlignmentOperation::X(att))
            }
            v if v == B_DISCRIMINANT => {
                ::core::option::Option::Some(MiniprotAlignmentOperation::B(att))
            }
            v if v == F_DISCRIMINANT => {
                ::core::option::Option::Some(MiniprotAlignmentOperation::F(att))
            }
            v if v == G_DISCRIMINANT => {
                ::core::option::Option::Some(MiniprotAlignmentOperation::G(att))
            }
            v if v == U_DISCRIMINANT => {
                ::core::option::Option::Some(MiniprotAlignmentOperation::U(att))
            }
            v if v == V_DISCRIMINANT => {
                ::core::option::Option::Some(MiniprotAlignmentOperation::V(att))
            }
            v if v == E_DISCRIMINANT => {
                ::core::option::Option::Some(MiniprotAlignmentOperation::E(att))
            }
            v if v == WARNING_DISCRIMINANT => {
                ::core::option::Option::Some(MiniprotAlignmentOperation::Warning)
            }
            _ => ::core::option::Option::None,
        }
    }

    pub fn get_value(&self) -> Option<&usize> {
        match self {
            MiniprotAlignmentOperation::Match(len) => Some(len),
            MiniprotAlignmentOperation::Ins(len) => Some(len),
            MiniprotAlignmentOperation::Del(len) => Some(len),
            MiniprotAlignmentOperation::N(len) => Some(len),
            MiniprotAlignmentOperation::Subst(len) => Some(len),
            MiniprotAlignmentOperation::H(len) => Some(len),
            MiniprotAlignmentOperation::P(len) => Some(len),
            MiniprotAlignmentOperation::Equal(len) => Some(len),
            MiniprotAlignmentOperation::X(len) => Some(len),
            MiniprotAlignmentOperation::B(len) => Some(len),
            MiniprotAlignmentOperation::F(len) => Some(len),
            MiniprotAlignmentOperation::G(len) => Some(len),
            MiniprotAlignmentOperation::U(len) => Some(len),
            MiniprotAlignmentOperation::V(len) => Some(len),
            MiniprotAlignmentOperation::E(len) => Some(len),
            MiniprotAlignmentOperation::Warning => None,
        }
    }
}

impl Default for MiniprotAlignmentOperation {
    fn default() -> Self {
        Self::Warning
    }
}

#[derive(Debug)]
pub struct MiniprotAlignment {
    // pub score: i32,
    // pub ystart: usize,
    // pub xstart: usize,
    // pub yend: usize,
    // pub xend: usize,
    // pub ylen: usize,
    // pub xlen: usize,
    // x is the query or read sequence and y is the reference or template sequence
    pub ystart: usize,
    pub yend: usize,
    pub operations: Vec<MiniprotAlignmentOperation>,
}

impl MiniprotAlignment {
    pub fn new(ystart: usize, yend: usize, operations: Vec<MiniprotAlignmentOperation>) -> Self {
        Self {
            ystart,
            yend,
            operations,
        }
    }

    pub fn to_cigar_str(&self) -> String {
        let cigar_str = self
            .operations
            .iter()
            .map(|code| {
                let cigar_len = code.get_value().unwrap();
                // #define NS_CIGAR_STR   "MIDNSHP=XBFGUVE"

                format!("{}{}", cigar_len, code)
            })
            .collect::<Vec<String>>()
            .join("");
        cigar_str
    }

    pub fn check_aln_len_bp(&self) -> (usize, usize) {
        let (match_count, mismatch_count) =
            self.operations
                .iter()
                .fold((0, 0), |(matches, mismatches), x| match x {
                    MiniprotAlignmentOperation::Match(len) => (matches + 3 * len, mismatches),
                    MiniprotAlignmentOperation::Ins(len) => (matches + 3 * len, mismatches),
                    MiniprotAlignmentOperation::Del(len) => (matches, mismatches + 3 * len),
                    MiniprotAlignmentOperation::N(len) => (matches + len, mismatches),
                    MiniprotAlignmentOperation::Subst(len) => (matches, mismatches), // Do nothing
                    MiniprotAlignmentOperation::H(len) => (matches, mismatches),     // Do nothing
                    MiniprotAlignmentOperation::P(len) => (matches, mismatches),     // Do nothing
                    MiniprotAlignmentOperation::Equal(len) => (matches, mismatches), // Do nothing
                    MiniprotAlignmentOperation::X(len) => (matches, mismatches),     // Do nothing
                    MiniprotAlignmentOperation::B(len) => (matches, mismatches),     // Do nothing
                    MiniprotAlignmentOperation::F(len) => (matches, mismatches + len), // Do nothing
                    MiniprotAlignmentOperation::G(len) => (matches + len, mismatches), // Do nothing
                    MiniprotAlignmentOperation::U(len) => (matches + len, mismatches), // Do nothing
                    MiniprotAlignmentOperation::V(len) => (matches + len, mismatches), // Do nothing
                    MiniprotAlignmentOperation::E(len) => (matches, mismatches),     // Do nothing
                    MiniprotAlignmentOperation::Warning => (matches, mismatches),
                });
        (match_count, mismatch_count)
    }
}

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

    pub fn builder_no_splice() -> Self {
        let aligner = Aligner {
            threads: 1,
            map_options: mp_mapopt_t::no_splice(),
            index_options: mp_idxopt_t::default(),
            index: None,
            reference_file: None,
            write_index: None,
            verbosity: 0,
        };

        aligner
    }

    // pub fn get_ctg_len(&self, ctg_name: String) -> i32 {
    //     (*(self.index.unwrap()).seq.offset(reg.rid as isize)).len as i128;
    // }
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
            assert!(
                mp_mapopt_check(&self.map_options as *const mp_mapopt_t) == 0,
                "Invalid map options"
            );
        }
    }

    /// Build the aligner index
    pub fn build(&mut self) {
        assert!(
            *MPSTART,
            "Miniprot has not been initialized. This should only happen once per run."
        );

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
    pub fn map(&self, seqname: &str, seq: &[u8]) -> Vec<Option<MiniprotAlignment>> {
        // n_regs are the number of results
        let mut n_regs: i32 = 0;
        // let mappings = BUF.with(|buf| {
        let buf = ThreadLocalBuffer::new();
        println!("here 1 - seq len {}", seq.len());

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

        // NO! Can't deref here in case n_regs == 0
        // let result: mp_reg1_t = unsafe { mp_reg.assume_init() };

        let mut mappings = Vec::<Option<MiniprotAlignment>>::with_capacity(n_regs as usize);

        // Need to format properly, see:
        // 	ctg = &mi->nt->ctg[r->vid>>1];
        // in format.c for a start (getting the contig name)

        for i in 0..n_regs {
            unsafe {
                let reg_ptr = (*mp_reg.as_ptr()).offset(i as isize);
                let const_ptr = reg_ptr as *const mp_reg1_t;
                let reg: mp_reg1_t = *reg_ptr;

                // let contig: *mut ::std::os::raw::c_char =
                //     (*(self.index.unwrap()).seq.offset(reg.rid as isize)).name;

                let is_primary = reg.parent == reg.id;
                let alignment = if !reg.p.is_null() {
                    let p = &*reg.p;
                    let feat = &*reg.feat;

                    // calculate the edit distance
                    // let nm = reg.blen - reg.mlen + p.n_ambi() as i32;
                    let n_cigar = p.n_cigar;

                    // number of frameshift events
                    let n_fs = p.n_fs;

                    // distance in bp to the closest stop codon
                    let dist_stop = p.dist_stop;

                    // distance in bp the the closest 'M'
                    let dist_start = p.dist_start;

                    // ternary operator to check if its mapped in reverse strand
                    // has_stop = (r->qe == seq->l_seq && r->p->dist_stop == 0);
                    // ve_mRNA = has_stop? r->ve + 3 : r->ve;
                    // vs = r->vid&1? ctg->len - ve_mRNA : r->vs;
                    // ve = r->vid&1? ctg->len - r->vs   : ve_mRNA;
                    let idx_seq = &self.index.unwrap();
                    // reg.id
                    let has_stop = (dist_stop == 0);
                    // let ctg_len = ((*idx_seq.nt).seq.offset(reg.id as isize)) as i32;
                    let ctg_len = (*(*idx_seq.nt).ctg.offset(reg.id as isize)).len as i64;
                    let ve_mRNA = if has_stop { reg.ve + 3 } else { reg.ve };
                    let y_start = if reg.vid == 1 {
                        ctg_len - ve_mRNA
                    } else {
                        reg.vs
                    };
                    // let y_start = reg.vs;
                    let y_end = if reg.vid == 1 {
                        ctg_len - reg.vs
                    } else {
                        ve_mRNA
                    };
                    // reg.

                    let x_start = reg.qs;
                    let x_end = reg.qe;

                    // alignment dp score
                    let feat_score = feat.score;

                    dbg!(reg.vid);

                    // let strand = if reg.rev() == 0 {
                    //     Strand::Forward
                    // } else {
                    //     Strand::Reverse
                    // };

                    // Create a vector of the cigar blocks
                    let cigar = if n_cigar > 0 {
                        let cigar_aln = p
                            .cigar
                            .as_slice(n_cigar as usize)
                            .to_vec()
                            .iter()
                            .map(|c| {
                                (MiniprotAlignmentOperation::from_repr(
                                    (c & 0xf) as usize,
                                    (c >> 4) as usize,
                                )
                                .unwrap())
                            }) // unpack the length and op code
                            .collect::<Vec<MiniprotAlignmentOperation>>();

                        Some(MiniprotAlignment::new(
                            y_start as usize,
                            y_end as usize,
                            cigar_aln,
                        ))
                        // let cigar_str = cigar
                        //     .iter()
                        //     .map(|(len, code)| {
                        //         // #define NS_CIGAR_STR   "MIDNSHP=XBFGUVE"
                        //         // let cigar_char = match code {
                        //         //     0 => "M",
                        //         //     1 => "I",
                        //         //     2 => "D",
                        //         //     3 => "N",
                        //         //     4 => "S",
                        //         //     5 => "H",
                        //         //     6 => "P",
                        //         //     7 => "=",
                        //         //     8 => "X",
                        //         //     9 => "B",
                        //         //     10 => "F",
                        //         //     11 => "G",
                        //         //     12 => "U",
                        //         //     13 => "V",
                        //         //     14 => "E",
                        //         //     _ => panic!("Invalid CIGAR code {code}"),
                        //         //     // _ => "E",
                        //         // };

                        //         format!("{}{}", len, code)
                        //     })
                        //     .collect::<Vec<String>>()
                        //     .join("");
                        // (Some(cigar), Some(cigar_str))
                    } else {
                        // (None, None)
                        None
                    };

                    cigar

                    // dbg!(cigar);
                    // dbg!(cigar_str);
                } else {
                    None
                };

                mappings.push(alignment);

                //         let (cs_str, md_str) = if cs || md {
                //             let mut cs_string: *mut libc::c_char = std::ptr::null_mut();
                //             let mut m_cs_string: libc::c_int = 0i32;

                //             let cs_str = if cs {
                //                 let _cs_len = mm_gen_cs(
                //                     km,
                //                     &mut cs_string,
                //                     &mut m_cs_string,
                //                     &self.idx.unwrap() as *const mm_idx_t,
                //                     const_ptr,
                //                     seq.as_ptr() as *const i8,
                //                     true.into(),
                //                 );
                //                 let _cs_string = std::ffi::CStr::from_ptr(cs_string)
                //                     .to_str()
                //                     .unwrap()
                //                     .to_string();
                //                 Some(_cs_string)
                //             } else {
                //                 None
                //             };

                //             let md_str = if md {
                //                 let _md_len = mm_gen_MD(
                //                     km,
                //                     &mut cs_string,
                //                     &mut m_cs_string,
                //                     &self.idx.unwrap() as *const mm_idx_t,
                //                     const_ptr,
                //                     seq.as_ptr() as *const i8,
                //                 );
                //                 let _md_string = std::ffi::CStr::from_ptr(cs_string)
                //                     .to_str()
                //                     .unwrap()
                //                     .to_string();
                //                 Some(_md_string)
                //             } else {
                //                 None
                //             };
                //             (cs_str, md_str)
                //         } else {
                //             (None, None)
                //         };

                //         Some(Alignment {
                //             nm,
                //             cigar,
                //             cigar_str,
                //             md: md_str,
                //             cs: cs_str,
                //         })
                //     } else {
                //         None
                //     };
                //     mappings.push(Mapping {
                //         target_name: Some(
                //             std::ffi::CStr::from_ptr(contig)
                //                 .to_str()
                //                 .unwrap()
                //                 .to_string(),
                //         ),
                //         target_len: (*(self.idx.unwrap()).seq.offset(reg.rid as isize)).len as i32,
                //         target_start: reg.rs,
                //         target_end: reg.re,
                //         query_name: None,
                //         query_len: NonZeroI32::new(seq.len() as i32),
                //         query_start: reg.qs,
                //         query_end: reg.qe,
                //         strand: if reg.rev() == 0 {
                //             Strand::Forward
                //         } else {
                //             Strand::Reverse
                //         },
                //         match_len: reg.mlen,
                //         block_len: reg.blen,
                //         mapq: reg.mapq(),
                //         is_primary,
                //         alignment,
                //     });
            }
        }

        // mappin gs
        // });

        // Ok(mappings)
        mappings

        // println!("Mappings: {}", mappings);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fffx::Fasta;
    use std::mem::MaybeUninit;

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

    #[test]
    fn test_simple_aligner_mapping() {
        let mut aligner = Aligner::builder_no_splice()
            .threads(1)
            .with_reference("test_data/LZP_9_and_10_A0A5D4N6A0.fsa")
            .verbosity(3);

        aligner.build();

        let working_seq = "MTVLWMAAVIVLICLNVIQFMIKKKRDRSLAYTAEQLGAMLNRDSARQILLHTDNQALRDLLVNINLFVEDRQQLSAQFAKTEHSMKRMLTNMSHDLKTPLTVILGYIETIQSDPDMPDEERERLLWKLGQKTNELIQMINSFFDLAKLESEDQDIPLTKIHMNEICKRNILHYYDAVQTKGFQAAIDIPDTPVYAQGNEEALDRILQNLLSNAIQYGAAGKFIGMTLSYDETSVAITVWDRGKGISETDQQRVFERLYTLEESRNKAFQGSGLGLTITKRLTEKMGGTISVHSKPYERTAFTITLKRMTY";
        let aln_res = aligner.map("Prot", working_seq.as_bytes());

        dbg!(&aln_res);
        dbg!(&aln_res.len());
        dbg!(aln_res[0].as_ref().unwrap().check_aln_len_bp());

        // let working_seq = "DPDQLRQHALAEGLTEEEYQAFLVYAAGVYSNMGNYKSFGDTKFVPNLPKDKL";
        // aligner.map("working seq2", working_seq.as_bytes());
    }

    #[test]
    fn test_simple_revaligner_mapping() {
        let mut aligner = Aligner::builder_no_splice()
            .threads(1)
            .with_reference("test_data/LZP_00103_fake_and_LZP_00103_P42405.fsa")
            .verbosity(3);

        aligner.build();

        let working_seq = "MELQLALDLVNIPEAIELVKEVEQYIDVVEIGTPVVINEGLRAVKEIKEAFPQLKVLADLKIMDAGGYEIMKASEAGADIITVLGATDDATIKGAVEEAKKQKKKILVDMINVKDIESRAKEIDALGVDYICVHTGYDLQAEGKNSFEELTTIKNTVKNAKTAIAGGIKLDTLPEVIQQKPDLVIVGGGITSAADKAETASKMKQLIVQG*";
        let aln_res = aligner.map("Prot", working_seq.as_bytes());

        dbg!(&aln_res);
        dbg!(&aln_res.len());
        dbg!(aln_res[0].as_ref().unwrap().check_aln_len_bp());
        dbg!(aln_res[0].as_ref().unwrap().to_cigar_str());

        // let working_seq = "DPDQLRQHALAEGLTEEEYQAFLVYAAGVYSNMGNYKSFGDTKFVPNLPKDKL";
        // aligner.map("working seq2", working_seq.as_bytes());
    }

    #[test]
    fn test_simple_aligner_cigar_str() {
        let mut aligner = Aligner::builder()
            .threads(1)
            .with_reference("miniprot-sys/miniprot/test/DPP3-hs.gen.fa.gz")
            .verbosity(3);

        aligner.build();

        let working_seq = "MADTQYILPNDIGVSSLDCREAFRLLSPTERLYAHHLSRAAWYGGLAVLLQTSPEAPYIYALLSRLFRAQDPDQLRQHALAEGLTEEEYQAFLVYAAGVYSNMGNYKSFGDTKFVPNLPKDKLGRVILGSKAAQQRPEEVRDLWQTCGDLMFSLEPRLRHLGLGKEGVTTYFSGDCTMEDAKLAQDFLDSQNLSAYNTRLFKVVGQEGKSHYEVRLASVLNTDPALDSELTSKLKRYEFQGNHFQVTRGDYAPILQKVVEHLEKAKAYAANSHQEQMLAQYVESFTQGSIEAHKRGSRFWIQDKGPIVESYIGFIESYRDPFGSRGEFEGFVAMVNKAMSAKFERLVASAEQLLKELPWPLAFEKDKFLTPDFTSLDVLTFAGSGIPAGINIPNYDDLRQTEGFKNVSLGNVLAVAYAAKREKLTFLEEEDKDLYIRWKGPSFDVQVGLHELLGHGSGKLFVQDEKGAFNFDKETVINPETGEQIQSWYRSGETWDSKFSTIASSYEECRAESVGLYLCLNPQVLEIFGFEGADAEDVIYVNWLNMVRAGLLALEFYTPEAANWRQAHMQARFVILRVLLEAGEGLVTVTPTTGSDGRPDARVRLDRSKIRSVGRPALERFLRRLQVLKSTGDVVAGRALYEGYAAVTDAPPECFLTLRDTVLLRKESRKLIVQPNTRLEGSEVQLVEYEASAAGLIRSFCERFPEDGPELEEVLIQLAAADARFWRNQAQEAPPGQA";
        let aln_res = aligner.map("working seq", working_seq.as_bytes());

        assert_eq!("90M2702N30M1272N46M590N25M571N31M3235U43M110N43M76V19M962U29M1D2M1I32M159U37M361N31M1572N21M96N41M143V40M1547N60M7134N7M1I2M1D44M4307U48M1I8M",
            aln_res[0].as_ref().unwrap().to_cigar_str()
        )

        // let working_seq = "DPDQLRQHALAEGLTEEEYQAFLVYAAGVYSNMGNYKSFGDTKFVPNLPKDKL";
        // aligner.map("working seq2", working_seq.as_bytes());
    }
}
