/* R .Call glue for the wickra-genome C ABI hub. */
#include <R.h>
#include <Rinternals.h>
#include <R_ext/Rdynload.h>
#include <stddef.h>
#include "wickra_genome.h"

/* --- handle lifetime ----------------------------------------------------- */

static void wkgenome_finalize(SEXP ext) {
    WickraGenome *h = (WickraGenome *)R_ExternalPtrAddr(ext);
    if (h) {
        wickra_genome_free(h);
    }
    R_ClearExternalPtr(ext);
}

static WickraGenome *handle_of(SEXP ext) {
    WickraGenome *h = (WickraGenome *)R_ExternalPtrAddr(ext);
    if (!h) {
        Rf_error("wickra-genome: handle is closed");
    }
    return h;
}

/* --- exported .Call entries ---------------------------------------------- */

SEXP wkgenome_version(void) {
    return Rf_mkString(wickra_genome_version());
}

SEXP wkgenome_new(SEXP spec_json) {
    const char *spec = CHAR(STRING_ELT(spec_json, 0));
    WickraGenome *h = wickra_genome_new(spec);
    if (!h) {
        Rf_error("wickra-genome: invalid spec");
    }
    SEXP ext = PROTECT(R_MakeExternalPtr(h, R_NilValue, R_NilValue));
    R_RegisterCFinalizerEx(ext, wkgenome_finalize, TRUE);
    UNPROTECT(1);
    return ext;
}

SEXP wkgenome_command(SEXP ext, SEXP cmd_json) {
    WickraGenome *h = handle_of(ext);
    const char *cmd = CHAR(STRING_ELT(cmd_json, 0));

    /* Length-out protocol: learn the length, then read into a caller buffer.
       Domain errors come back in-band as {"ok":false,...} JSON, not a negative
       code; only unusable arguments / a caught panic return < 0. */
    int len = wickra_genome_command(h, cmd, NULL, 0);
    if (len < 0) {
        Rf_error("wickra-genome: command failed (code %d)", len);
    }
    char *buf = (char *)R_alloc((size_t)len + 1, 1);
    wickra_genome_command(h, cmd, buf, (size_t)len + 1);
    return Rf_mkString(buf);
}

/* --- registration -------------------------------------------------------- */

static const R_CallMethodDef CallEntries[] = {
    {"wkgenome_version", (DL_FUNC)&wkgenome_version, 0},
    {"wkgenome_new", (DL_FUNC)&wkgenome_new, 1},
    {"wkgenome_command", (DL_FUNC)&wkgenome_command, 2},
    {NULL, NULL, 0}};

void R_init_wickragenome(DllInfo *dll) {
    R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
    R_useDynamicSymbols(dll, FALSE);
}
