package org.wickra.genome;

import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import org.junit.jupiter.api.Test;

class GenomeTest {
    static final String SPEC =
            "{\"features\":[{\"kind\":\"price\",\"field\":\"close\"}],"
                    + "\"symbols\":[\"AAA\",\"BBB\",\"CCC\"],"
                    + "\"normalize\":\"z_score\",\"metric\":\"euclid\",\"seed\":24333}";

    static final String DATA =
            "{\"AAA\":[{\"time\":0,\"open\":1,\"high\":1,\"low\":1,\"close\":1,\"volume\":0}],"
                    + "\"BBB\":[{\"time\":0,\"open\":2,\"high\":2,\"low\":2,\"close\":2,\"volume\":0}],"
                    + "\"CCC\":[{\"time\":0,\"open\":100,\"high\":100,\"low\":100,\"close\":100,\"volume\":0}]}";

    private static Genome built() {
        Genome g = new Genome(SPEC);
        String ok = g.command("{\"cmd\":\"build\",\"data\":" + DATA + "}");
        assertTrue(ok.contains("\"ok\":true"), ok);
        return g;
    }

    @Test
    void versionIsNonEmpty() {
        assertFalse(Genome.version().isEmpty());
    }

    @Test
    void similarExcludesSelf() {
        try (Genome g = built()) {
            String out = g.command("{\"cmd\":\"similar\",\"symbol\":\"AAA\",\"k\":2}");
            assertTrue(out.startsWith("{\"neighbors\":[{\"symbol\":\"BBB\""), out);
        }
    }

    @Test
    void anomalyRanksOutlierFirst() {
        try (Genome g = built()) {
            String out = g.command("{\"cmd\":\"anomaly\"}");
            assertTrue(out.startsWith("{\"anomalies\":[{\"symbol\":\"CCC\""), out);
        }
    }

    @Test
    void invalidSpecThrows() {
        assertThrows(IllegalArgumentException.class, () -> new Genome("{ not valid json"));
    }
}
