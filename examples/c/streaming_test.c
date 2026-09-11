/* Streaming equals batch, through the C ABI's two-call idiom.
 *
 * wickra-genome-core proves this in Rust, but that says nothing about the boundary a C
 * caller crosses. Every reach behind this ABI asks for the response length first
 * and reads it second, so a command that is not a pure function of its payload
 * runs twice per call — and `feed` is exactly that: it mutates the symbol's
 * rolling state. A double-applied feed would fold every candle in twice and
 * nothing outside this test would notice.
 *
 * The dataset is inline rather than read from golden/, because C carries no JSON
 * parser and splitting the corpus by hand would test the splitter. What matters
 * is that the same bars reach the core two ways.
 *
 * The spec uses Sma(3), which has a lookback: an axis over the raw close reads
 * the same however many times a bar arrived, which is precisely how this class
 * of bug stays invisible.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "wickra_genome.h"

static const char *SPEC =
    "{\"features\":[{\"kind\":\"indicator\",\"name\":\"Sma\",\"params\":[3]},"
    "{\"kind\":\"price\",\"field\":\"close\"}],\"symbols\":[\"AAA\",\"BBB\"],"
    "\"normalize\":\"z_score\",\"metric\":\"euclid\",\"seed\":24333}";

/* Six bars: three per symbol, a rising ramp so Sma(3) is well defined. */
static const char *DATASET =
    "{\"AAA\":["
    "{\"time\":1,\"open\":10,\"high\":10,\"low\":10,\"close\":10,\"volume\":1},"
    "{\"time\":2,\"open\":20,\"high\":20,\"low\":20,\"close\":20,\"volume\":1},"
    "{\"time\":3,\"open\":30,\"high\":30,\"low\":30,\"close\":30,\"volume\":1}],"
    "\"BBB\":["
    "{\"time\":1,\"open\":40,\"high\":40,\"low\":40,\"close\":40,\"volume\":1},"
    "{\"time\":2,\"open\":50,\"high\":50,\"low\":50,\"close\":50,\"volume\":1},"
    "{\"time\":3,\"open\":60,\"high\":60,\"low\":60,\"close\":60,\"volume\":1}]}";

static const char *FEEDS[] = {
    "{\"cmd\":\"feed\",\"symbol\":\"AAA\",\"candle\":{\"time\":1,\"open\":10,\"high\":10,\"low\":10,\"close\":10,\"volume\":1}}",
    "{\"cmd\":\"feed\",\"symbol\":\"AAA\",\"candle\":{\"time\":2,\"open\":20,\"high\":20,\"low\":20,\"close\":20,\"volume\":1}}",
    "{\"cmd\":\"feed\",\"symbol\":\"AAA\",\"candle\":{\"time\":3,\"open\":30,\"high\":30,\"low\":30,\"close\":30,\"volume\":1}}",
    "{\"cmd\":\"feed\",\"symbol\":\"BBB\",\"candle\":{\"time\":1,\"open\":40,\"high\":40,\"low\":40,\"close\":40,\"volume\":1}}",
    "{\"cmd\":\"feed\",\"symbol\":\"BBB\",\"candle\":{\"time\":2,\"open\":50,\"high\":50,\"low\":50,\"close\":50,\"volume\":1}}",
    "{\"cmd\":\"feed\",\"symbol\":\"BBB\",\"candle\":{\"time\":3,\"open\":60,\"high\":60,\"low\":60,\"close\":60,\"volume\":1}}",
};
static const size_t FEED_COUNT = sizeof(FEEDS) / sizeof(FEEDS[0]);

static const char *QUERIES[] = {
    "{\"cmd\":\"vector\",\"symbol\":\"AAA\"}",
    "{\"cmd\":\"similar\",\"symbol\":\"AAA\",\"k\":1}",
    "{\"cmd\":\"cluster\",\"k\":2}",
    "{\"cmd\":\"anomaly\"}",
};
static const size_t QUERY_COUNT = sizeof(QUERIES) / sizeof(QUERIES[0]);

/* Run one command through the documented two-call idiom. Caller frees. */
static char *run(WickraGenome *genome, const char *cmd) {
    int32_t len = wickra_genome_command(genome, cmd, NULL, 0);
    if (len < 0) {
        fprintf(stderr, "command failed: code %d\n", (int)len);
        return NULL;
    }
    char *buf = (char *)malloc((size_t)len + 1);
    if (!buf) {
        return NULL;
    }
    int32_t written = wickra_genome_command(genome, cmd, buf, (size_t)len + 1);
    if (written != len) {
        fprintf(stderr, "second call returned %d, first said %d\n", (int)written, (int)len);
        free(buf);
        return NULL;
    }
    return buf;
}

int main(void) {
    char command[4096];
    int n = snprintf(command, sizeof(command), "{\"cmd\":\"build\",\"data\":%s}", DATASET);
    if (n < 0 || (size_t)n >= sizeof(command)) {
        fprintf(stderr, "build command did not fit\n");
        return 1;
    }

    WickraGenome *batch = wickra_genome_new(SPEC);
    WickraGenome *streaming = wickra_genome_new(SPEC);
    if (!batch || !streaming) {
        fprintf(stderr, "failed to build a genome from the spec\n");
        wickra_genome_free(batch);
        wickra_genome_free(streaming);
        return 1;
    }

    int failures = 0;
    char *ack = run(batch, command);
    if (!ack) {
        failures++;
    }
    free(ack);

    for (size_t i = 0; i < FEED_COUNT; i++) {
        char *fed = run(streaming, FEEDS[i]);
        if (!fed) {
            failures++;
        }
        free(fed);
    }

    for (size_t i = 0; i < QUERY_COUNT && failures == 0; i++) {
        char *want = run(batch, QUERIES[i]);
        char *got = run(streaming, QUERIES[i]);
        if (!want || !got) {
            failures++;
        } else if (strcmp(want, got) != 0) {
            fprintf(stderr, "streaming != batch for %s\n  batch    : %s\n  streaming: %s\n",
                    QUERIES[i], want, got);
            failures++;
        } else if (i == 0) {
            /* Sma(3) over 10, 20, 30 is 20. Fed twice each it would be the mean
             * of 20, 30, 30 instead, so this value tells the two apart. */
            printf("vector: %s\n", want);
            if (strstr(want, "\"values\":[20.0,") == NULL &&
                strstr(want, "\"values\":[20,") == NULL) {
                fprintf(stderr, "expected Sma(3) of 20 from three bars\n");
                failures++;
            }
        }
        free(want);
        free(got);
    }

    wickra_genome_free(batch);
    wickra_genome_free(streaming);

    if (failures > 0) {
        fprintf(stderr, "%d check(s) failed\n", failures);
        return 1;
    }
    printf("streaming equals batch\n");
    return 0;
}
