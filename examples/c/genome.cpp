// A runnable C++ example: build a genome over a tiny three-symbol universe
// through the wickra-genome C ABI and print the nearest-neighbour response.
#include <cstdio>
#include <string>
#include <vector>

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

static bool run(WickraGenome *genome, const char *cmd) {
    int len = wickra_genome_command(genome, cmd, nullptr, 0);
    if (len < 0) {
        std::fprintf(stderr, "command failed: code %d\n", len);
        return false;
    }
    std::vector<char> buf(static_cast<size_t>(len) + 1);
    wickra_genome_command(genome, cmd, buf.data(), buf.size());
    std::printf("%s\n", buf.data());
    return true;
}

int main() {
    WickraGenome *genome = wickra_genome_new(SPEC);
    if (!genome) {
        std::fprintf(stderr, "failed to build genome\n");
        return 1;
    }
    if (!run(genome, BUILD_CMD)) {
        wickra_genome_free(genome);
        return 1;
    }
    std::printf("wickra-genome %s\n", wickra_genome_version());
    std::printf("AAA neighbours: ");
    if (!run(genome, SIMILAR_CMD)) {
        wickra_genome_free(genome);
        return 1;
    }
    wickra_genome_free(genome);
    return 0;
}
