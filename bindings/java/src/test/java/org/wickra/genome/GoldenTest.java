package org.wickra.genome;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

import org.junit.jupiter.api.Test;

// Determinism: the same commands yield the byte-identical response string. The
// full cross-language golden (asserting the response equals a blessed
// golden/expected file) lands with the golden corpus in P-GEN-4; here we pin the
// core invariant that the results are byte-reproducible, which every binding
// must preserve by forwarding the command string verbatim.
class GoldenTest {
    private static String run(String cmd) {
        try (Genome g = new Genome(GenomeTest.SPEC)) {
            g.command("{\"cmd\":\"build\",\"data\":" + GenomeTest.DATA + "}");
            return g.command(cmd);
        }
    }

    @Test
    void sameCommandsSameResponse() {
        String cmd = "{\"cmd\":\"cluster\",\"k\":2}";
        assertEquals(run(cmd), run(cmd));
    }

    @Test
    void anomalyFieldOrderIsSymbolFirst() {
        assertTrue(run("{\"cmd\":\"anomaly\"}").startsWith("{\"anomalies\":[{\"symbol\":"));
    }
}
