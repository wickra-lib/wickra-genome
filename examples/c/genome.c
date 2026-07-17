/* A runnable C example: build a genome over a tiny three-symbol universe through
 * the wickra-genome C ABI and print the nearest-neighbour response. Every
 * language example runs the same request and prints the same summary. */
#include <stdio.h>
#include <stdlib.h>

#include "wickra_genome.h"

static const char *SPEC =
    "{\"features\":[{\"kind\":\"price\",\"field\":\"close\"}],"
    "\"symbols\":[\"AAA\",\"BBB\",\"CCC\"],\"normalize\":\"z_score\","
    "\"metric\":\"euclid\",\"seed\":24333}";

static const char *BUILD_CMD =
    "{\"cmd\":\"build\",\"data\":{"
    "\"AAA\":[{\"time\":0,\"open\":1,\"high\":1,\"low\":1,\"close\":1,\"volume\":0}],"
    "\"BBB\":[{\"time\":0,\"open\":2,\"high\":2,\"low\":2,\"close\":2,\"volume\":0}],"
    "\"CCC\":[{\"time\":0,\"open\":100,\"high\":100,\"low\":100,\"close\":100,\"volume\":0}]}}";

static const char *SIMILAR_CMD = "{\"cmd\":\"similar\",\"symbol\":\"AAA\",\"k\":2}";

/* Run a command through the two-call length-out protocol and print the response. */
static int run(WickraGenome *genome, const char *cmd) {
    int len = wickra_genome_command(genome, cmd, NULL, 0);
    if (len < 0) {
        fprintf(stderr, "command failed: code %d\n", len);
        return -1;
    }
    char *buf = (char *)malloc((size_t)len + 1);
    if (!buf) {
        return -1;
    }
    wickra_genome_command(genome, cmd, buf, (size_t)len + 1);
    printf("%s\n", buf);
    free(buf);
    return len;
}

int main(void) {
    WickraGenome *genome = wickra_genome_new(SPEC);
    if (!genome) {
        fprintf(stderr, "failed to build genome\n");
        return 1;
    }
    if (run(genome, BUILD_CMD) < 0) {
        wickra_genome_free(genome);
        return 1;
    }
    printf("wickra-genome %s\n", wickra_genome_version());
    printf("AAA neighbours: ");
    if (run(genome, SIMILAR_CMD) < 0) {
        wickra_genome_free(genome);
        return 1;
    }
    wickra_genome_free(genome);
    return 0;
}
