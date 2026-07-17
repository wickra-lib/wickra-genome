// A runnable C# example: build a genome over a tiny three-symbol universe and
// print the nearest neighbour and the biggest outlier.
//
//   dotnet run --project examples/csharp/Genome
//
// Every language example runs the same request and prints the same summary —
// that is the cross-language guarantee.
using System.Text.Json;
using Wickra.Genome;

// A price-close genome over three symbols: AAA and BBB sit close together, CCC is
// far away — so AAA's nearest neighbour is BBB and CCC leads the anomaly ranking.
const string spec =
    "{\"features\":[{\"kind\":\"price\",\"field\":\"close\"}]," +
    "\"symbols\":[\"AAA\",\"BBB\",\"CCC\"],\"normalize\":\"z_score\"," +
    "\"metric\":\"euclid\",\"seed\":24333}";

const string buildCmd =
    "{\"cmd\":\"build\",\"data\":{" +
    "\"AAA\":[{\"time\":0,\"open\":1,\"high\":1,\"low\":1,\"close\":1,\"volume\":0}]," +
    "\"BBB\":[{\"time\":0,\"open\":2,\"high\":2,\"low\":2,\"close\":2,\"volume\":0}]," +
    "\"CCC\":[{\"time\":0,\"open\":100,\"high\":100,\"low\":100,\"close\":100,\"volume\":0}]}}";

using var genome = new Genome(spec);
genome.Command(buildCmd);

JsonElement similar = JsonDocument.Parse(
    genome.Command("{\"cmd\":\"similar\",\"symbol\":\"AAA\",\"k\":2}")).RootElement;
JsonElement anomaly = JsonDocument.Parse(
    genome.Command("{\"cmd\":\"anomaly\"}")).RootElement;

Console.WriteLine($"wickra-genome {Genome.Version()}");
Console.WriteLine($"AAA nearest: {similar.GetProperty("neighbors")[0].GetProperty("symbol").GetString()}");
Console.WriteLine($"top anomaly: {anomaly.GetProperty("anomalies")[0].GetProperty("symbol").GetString()}");
