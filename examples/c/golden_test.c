/* Cross-language golden parity, from C.
 *
 * Build the genome from each committed golden/specs/*.json over the committed
 * universe, run each of the four queries, and assert the response equals
 * golden/expected/<op>/<spec>.json byte-for-byte. The ABI returns the core's
 * compact command output verbatim, so byte equality is the exact cross-language
 * parity check — the same one Python, Node, Go, C#, Java, R and WASM make.
 *
 * C has no directory API that is portable between POSIX and Windows, so the spec
 * list is globbed by CMake at configure time and written into golden_specs.h.
 * That keeps the property the other bindings get from a runtime glob: a spec
 * added to the corpus is covered here without editing this file. A
 * hand-maintained list would silently skip it, which is the failure this whole
 * corpus exists to prevent.
 *
 * The universe is read from golden/data.json rather than the per-symbol CSVs:
 * the ABI speaks JSON, and a CSV parser here would be a second implementation of
 * the loader rather than a test of the engine.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "wickra_genome.h"

#include "golden_specs.h" /* GOLDEN_DIR, GOLDEN_SPECS, GOLDEN_SPEC_COUNT */

/* The four queries every spec is asked, and the expected/ subdirectory each
 * answer is pinned in. `cluster5` asks for five clusters; every other spec
 * asks for three, which is what the Rust harness does. */
static const char *OPS[] = {"vector", "similar", "cluster", "anomaly"};
static const size_t OP_COUNT = sizeof(OPS) / sizeof(OPS[0]);

static const char *query_for(const char *op, const char *spec_name) {
    if (strcmp(op, "vector") == 0) {
        return "{\"cmd\":\"vector\",\"symbol\":\"AAA\"}";
    }
    if (strcmp(op, "similar") == 0) {
        return "{\"cmd\":\"similar\",\"symbol\":\"AAA\",\"k\":3}";
    }
    if (strcmp(op, "cluster") == 0) {
        return strncmp(spec_name, "cluster5", 8) == 0 ? "{\"cmd\":\"cluster\",\"k\":5}"
                                                      : "{\"cmd\":\"cluster\",\"k\":3}";
    }
    return "{\"cmd\":\"anomaly\"}";
}

/* Read a whole file. Caller frees. Returns NULL and reports on failure. */
static char *slurp(const char *path) {
    FILE *file = fopen(path, "rb");
    if (!file) {
        fprintf(stderr, "cannot open %s\n", path);
        return NULL;
    }
    if (fseek(file, 0, SEEK_END) != 0) {
        fclose(file);
        return NULL;
    }
    long size = ftell(file);
    if (size < 0 || fseek(file, 0, SEEK_SET) != 0) {
        fclose(file);
        return NULL;
    }
    char *buf = (char *)malloc((size_t)size + 1);
    if (!buf) {
        fclose(file);
        return NULL;
    }
    size_t got = fread(buf, 1, (size_t)size, file);
    fclose(file);
    buf[got] = '\0';
    return buf;
}

/* Trim ASCII whitespace in place and return the start of the trimmed text. */
static char *trim(char *text) {
    while (*text == ' ' || *text == '\n' || *text == '\r' || *text == '\t') {
        text++;
    }
    size_t len = strlen(text);
    while (len > 0) {
        char last = text[len - 1];
        if (last != ' ' && last != '\n' && last != '\r' && last != '\t') {
            break;
        }
        text[--len] = '\0';
    }
    return text;
}

static char *join(const char *a, const char *b, const char *c) {
    size_t len = strlen(a) + strlen(b) + strlen(c) + 1;
    char *out = (char *)malloc(len);
    if (out) {
        snprintf(out, len, "%s%s%s", a, b, c);
    }
    return out;
}

/* Run one command through the two-call idiom. Caller frees. */
static char *run(WickraGenome *genome, const char *cmd) {
    int32_t len = wickra_genome_command(genome, cmd, NULL, 0);
    if (len < 0) {
        return NULL;
    }
    char *buf = (char *)malloc((size_t)len + 1);
    if (!buf) {
        return NULL;
    }
    if (wickra_genome_command(genome, cmd, buf, (size_t)len + 1) != len) {
        free(buf);
        return NULL;
    }
    return buf;
}

int main(void) {
    if (GOLDEN_SPEC_COUNT == 0) {
        fprintf(stderr, "no golden specs were configured; this would test nothing\n");
        return 1;
    }

    char *raw_data = slurp(GOLDEN_DIR "/data.json");
    if (!raw_data) {
        return 1;
    }
    char *data = trim(raw_data);

    int failures = 0;
    size_t checked = 0;
    for (size_t i = 0; i < GOLDEN_SPEC_COUNT; i++) {
        const char *name = GOLDEN_SPECS[i];

        char *spec_path = join(GOLDEN_DIR "/specs/", name, "");
        char *spec = spec_path ? slurp(spec_path) : NULL;
        free(spec_path);
        if (!spec) {
            fprintf(stderr, "%s: missing spec\n", name);
            failures++;
            continue;
        }

        char *build_cmd = join("{\"cmd\":\"build\",\"data\":", data, "}");
        for (size_t op = 0; op < OP_COUNT; op++) {
            char *expected_dir = join(GOLDEN_DIR "/expected/", OPS[op], "/");
            char *expected_path = expected_dir ? join(expected_dir, name, "") : NULL;
            free(expected_dir);
            char *expected_raw = expected_path ? slurp(expected_path) : NULL;
            free(expected_path);
            if (!expected_raw) {
                /* Not every op is pinned for every spec; skip rather than fail. */
                continue;
            }
            char *expected = trim(expected_raw);

            WickraGenome *genome = wickra_genome_new(spec);
            char *ack = genome && build_cmd ? run(genome, build_cmd) : NULL;
            char *got_raw = ack ? run(genome, query_for(OPS[op], name)) : NULL;
            if (!got_raw) {
                fprintf(stderr, "%s/%s: command failed\n", OPS[op], name);
                failures++;
            } else {
                char *got = trim(got_raw);
                if (strcmp(got, expected) != 0) {
                    fprintf(stderr, "%s/%s: mismatch\n  expected: %s\n  got:      %s\n",
                            OPS[op], name, expected, got);
                    failures++;
                } else {
                    checked++;
                }
            }
            free(got_raw);
            free(ack);
            wickra_genome_free(genome);
            free(expected_raw);
        }
        free(build_cmd);
        free(spec);
    }

    free(raw_data);

    if (failures > 0) {
        fprintf(stderr, "%d golden comparison(s) did not match\n", failures);
        return 1;
    }
    printf("all %zu golden comparisons are byte-identical from C\n", checked);
    return 0;
}
