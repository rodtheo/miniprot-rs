#include "zlib.h"
#include "miniprot/nasw.h"
#include "miniprot/kalloc.h"
#include "miniprot/ketopt.h"
#include "miniprot/kseq.h"
#include "miniprot/ksort.h"
#include "miniprot/kthread.h"
#include "miniprot/kvec-km.h"
#include "miniprot/miniprot.h"
#include "miniprot/mppriv.h"

mp_tbuf_t *mp_tbuf_init(void);
void mp_tbuf_destroy(mp_tbuf_t *b);
mp_reg1_t *mp_map(const mp_idx_t *mi, int qlen, const char *seq, int *n_reg, mp_tbuf_t *b, const mp_mapopt_t *opt, const char *qname);