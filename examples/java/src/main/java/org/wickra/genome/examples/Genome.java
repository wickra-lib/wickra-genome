package org.wickra.genome.examples;

/**
 * A runnable Java example: build a genome over a tiny three-symbol universe and
 * print the nearest neighbour and the biggest outlier.
 *
 * <pre>
 *   mvn -q compile exec:java -Dexec.mainClass=org.wickra.genome.examples.Genome
 * </pre>
 *
 * Every language example runs the same request and prints the same summary — that
 * is the cross-language guarantee.
 */
public final class Genome {
    private Genome() {}

    // A price-close genome over three symbols: AAA and BBB sit close together, CCC
    // is far away — so AAA's nearest neighbour is BBB and CCC leads the anomaly
    // ranking.
    private static final String SPEC =
            "{\"features\":[{\"kind\":\"price\",\"field\":\"close\"}],"
                    + "\"symbols\":[\"AAA\",\"BBB\",\"CCC\"],\"normalize\":\"z_score\","
                    + "\"metric\":\"euclid\",\"seed\":24333}";

    private static final String BUILD_CMD =
            "{\"cmd\":\"build\",\"data\":{"
                    + "\"AAA\":[{\"time\":0,\"open\":1,\"high\":1,\"low\":1,\"close\":1,\"volume\":0}],"
                    + "\"BBB\":[{\"time\":0,\"open\":2,\"high\":2,\"low\":2,\"close\":2,\"volume\":0}],"
                    + "\"CCC\":[{\"time\":0,\"open\":100,\"high\":100,\"low\":100,\"close\":100,\"volume\":0}]}}";

    // The first `"symbol":"X"` in a response — the nearest neighbour or the top
    // outlier, both of which lead their sorted list.
    private static String firstSymbol(String json) {
        return json.split("\"symbol\":\"")[1].split("\"")[0];
    }

    public static void main(String[] args) {
        try (org.wickra.genome.Genome genome = new org.wickra.genome.Genome(SPEC)) {
            genome.command(BUILD_CMD);
            String similar = genome.command("{\"cmd\":\"similar\",\"symbol\":\"AAA\",\"k\":2}");
            String anomaly = genome.command("{\"cmd\":\"anomaly\"}");

            System.out.printf("wickra-genome %s%n", org.wickra.genome.Genome.version());
            System.out.printf("AAA nearest: %s%n", firstSymbol(similar));
            System.out.printf("top anomaly: %s%n", firstSymbol(anomaly));
        }
    }
}
