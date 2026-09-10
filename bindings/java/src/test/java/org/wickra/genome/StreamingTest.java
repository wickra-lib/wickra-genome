package org.wickra.genome;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

import org.junit.jupiter.api.Test;

/**
 * Streaming equals batch, driven through the JSON command boundary.
 *
 * <p>genome-core proves this in Rust, but that says nothing about the boundary
 * this binding crosses. Every other test here only ever sends {@code build}, so
 * {@code feed} was exercised in no language at all: a binding that mis-serialised
 * a candle on the feed path had no test to fail.
 *
 * <p>The ramp is chosen so a candle applied twice would change {@code Sma(3)},
 * which is what tells the two paths apart.
 */
class StreamingTest {
    private static final String SPEC =
            "{\"features\":[{\"kind\":\"indicator\",\"name\":\"Sma\",\"params\":[3]},"
                    + "{\"kind\":\"price\",\"field\":\"close\"}],"
                    + "\"symbols\":[\"AAA\",\"BBB\"],"
                    + "\"normalize\":\"z_score\",\"metric\":\"euclid\",\"seed\":24333}";

    private static final String[] SYMBOLS = {"AAA", "BBB"};
    private static final double[][] BARS = {{10, 20, 30}, {40, 50, 60}};

    private static String candle(int ts, double close) {
        return "{\"time\":" + ts + ",\"open\":" + close + ",\"high\":" + close
                + ",\"low\":" + close + ",\"close\":" + close + ",\"volume\":1}";
    }

    private static String data() {
        StringBuilder out = new StringBuilder("{");
        for (int s = 0; s < SYMBOLS.length; s++) {
            if (s > 0) {
                out.append(',');
            }
            out.append('"').append(SYMBOLS[s]).append("\":[");
            for (int b = 0; b < BARS[s].length; b++) {
                if (b > 0) {
                    out.append(',');
                }
                out.append(candle(b + 1, BARS[s][b]));
            }
            out.append(']');
        }
        return out.append('}').toString();
    }

    private static void feedAll(Genome genome) {
        for (int s = 0; s < SYMBOLS.length; s++) {
            for (int b = 0; b < BARS[s].length; b++) {
                genome.command("{\"cmd\":\"feed\",\"symbol\":\"" + SYMBOLS[s]
                        + "\",\"candle\":" + candle(b + 1, BARS[s][b]) + "}");
            }
        }
    }

    @Test
    void streamingEqualsBatch() {
        Genome batch = new Genome(SPEC);
        assertTrue(batch.command("{\"cmd\":\"build\",\"data\":" + data() + "}")
                .contains("\"ok\":true"));

        Genome streaming = new Genome(SPEC);
        feedAll(streaming);

        String[] queries = {
            "{\"cmd\":\"vector\",\"symbol\":\"AAA\"}",
            "{\"cmd\":\"similar\",\"symbol\":\"AAA\",\"k\":1}",
            "{\"cmd\":\"cluster\",\"k\":2}",
            "{\"cmd\":\"anomaly\"}",
        };
        for (String query : queries) {
            assertEquals(batch.command(query), streaming.command(query),
                    "streaming != batch for " + query);
        }
    }

    @Test
    void batchVectorIsTheMeanOfThreeBars() {
        Genome genome = new Genome(SPEC);
        genome.command("{\"cmd\":\"build\",\"data\":" + data() + "}");
        String vector = genome.command("{\"cmd\":\"vector\",\"symbol\":\"AAA\"}");
        assertTrue(vector.contains("\"values\":[20.0,"),
                "Sma(3) over 10, 20, 30 is 20: " + vector);
    }

    @Test
    void resetReturnsToThePreFeedState() {
        Genome genome = new Genome(SPEC);
        String query = "{\"cmd\":\"cluster\",\"k\":2}";
        String empty = genome.command(query);

        feedAll(genome);
        assertNotEquals(empty, genome.command(query),
                "feeding the whole universe changed nothing");

        genome.command("{\"cmd\":\"reset\"}");
        assertEquals(empty, genome.command(query),
                "reset did not return the genome to its pre-feed state");
    }
}
