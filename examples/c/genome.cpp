// A runnable C++ example: build a genome over a tiny three-symbol universe and
// print the nearest-neighbour response.
//
// This goes through `wickra_genome.hpp`, the C++ hull shipped beside the C
// header, because that hull is what a C++ caller is meant to use: it owns and
// frees the handle, runs the two-call length protocol behind
// `wickra_genome_command` for you, and turns a refusal into an exception rather
// than a negative integer that is easy to ignore. Calling the C functions
// directly from C++ works too -- `genome.c` shows that -- but then the hull
// would be shipped without anything building it.
#include <cstdio>
#include <string>

#include "wickra_genome.hpp"

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

int main() {
    try {
        wickra::Genome genome(SPEC);
        std::printf("%s\n", genome.command(BUILD_CMD).c_str());
        std::printf("wickra-genome %s\n", wickra::Genome::version().c_str());
        std::printf("AAA neighbours: %s\n", genome.command(SIMILAR_CMD).c_str());
    } catch (const wickra::GenomeError &err) {
        // Every failure arrives here: a spec the core rejects, a command it does
        // not know, a response that changed length between the two ABI calls.
        std::fprintf(stderr, "%s\n", err.what());
        return 1;
    }
    return 0;
}
