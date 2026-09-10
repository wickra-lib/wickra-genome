using System.Globalization;
using System.Text;
using System.Text.Json;
using Wickra.Genome;
using Xunit;

namespace WickraGenome.Tests;

/// <summary>
/// Streaming equals batch, driven through the JSON command boundary.
///
/// genome-core proves this in Rust, but that says nothing about the boundary
/// this binding crosses. Every other test here only ever sends
/// <c>{"cmd":"build"}</c>, so <c>feed</c> was exercised in no language at all: a
/// binding that mis-serialised a candle on the feed path had no test to fail.
///
/// The ramp is chosen so a candle applied twice would change <c>Sma(3)</c>,
/// which is what tells the two paths apart.
/// </summary>
public class StreamingTests
{
    private const string StreamSpec =
        "{\"features\":[{\"kind\":\"indicator\",\"name\":\"Sma\",\"params\":[3]}," +
        "{\"kind\":\"price\",\"field\":\"close\"}],\"symbols\":[\"AAA\",\"BBB\"]," +
        "\"normalize\":\"z_score\",\"metric\":\"euclid\",\"seed\":24333}";

    private static readonly string[] Symbols = { "AAA", "BBB" };
    private static readonly double[][] Bars =
    {
        new double[] { 10, 20, 30 },
        new double[] { 40, 50, 60 },
    };

    private static string Candle(int ts, double close)
    {
        string c = close.ToString(CultureInfo.InvariantCulture);
        return "{\"time\":" + ts.ToString(CultureInfo.InvariantCulture) +
               ",\"open\":" + c + ",\"high\":" + c + ",\"low\":" + c +
               ",\"close\":" + c + ",\"volume\":1}";
    }

    private static string Data()
    {
        var sb = new StringBuilder("{");
        for (int s = 0; s < Symbols.Length; s++)
        {
            if (s > 0)
            {
                sb.Append(',');
            }

            sb.Append('"').Append(Symbols[s]).Append("\":[");
            for (int b = 0; b < Bars[s].Length; b++)
            {
                if (b > 0)
                {
                    sb.Append(',');
                }

                sb.Append(Candle(b + 1, Bars[s][b]));
            }

            sb.Append(']');
        }

        return sb.Append('}').ToString();
    }

    private static void FeedAll(Genome genome)
    {
        for (int s = 0; s < Symbols.Length; s++)
        {
            for (int b = 0; b < Bars[s].Length; b++)
            {
                genome.Command("{\"cmd\":\"feed\",\"symbol\":\"" + Symbols[s] +
                               "\",\"candle\":" + Candle(b + 1, Bars[s][b]) + "}");
            }
        }
    }

    [Fact]
    public void StreamingEqualsBatch()
    {
        using var batch = new Genome(StreamSpec);
        Assert.Contains("\"ok\":true", batch.Command("{\"cmd\":\"build\",\"data\":" + Data() + "}"));

        using var streaming = new Genome(StreamSpec);
        FeedAll(streaming);

        string[] queries =
        {
            "{\"cmd\":\"vector\",\"symbol\":\"AAA\"}",
            "{\"cmd\":\"similar\",\"symbol\":\"AAA\",\"k\":1}",
            "{\"cmd\":\"cluster\",\"k\":2}",
            "{\"cmd\":\"anomaly\"}",
        };
        foreach (string query in queries)
        {
            Assert.Equal(batch.Command(query), streaming.Command(query));
        }
    }

    [Fact]
    public void BatchVectorIsTheMeanOfThreeBars()
    {
        using var genome = new Genome(StreamSpec);
        genome.Command("{\"cmd\":\"build\",\"data\":" + Data() + "}");
        string vector = genome.Command("{\"cmd\":\"vector\",\"symbol\":\"AAA\"}");
        using var doc = JsonDocument.Parse(vector);
        double first = doc.RootElement.GetProperty("values")[0].GetDouble();
        Assert.Equal(20.0, first);
    }

    [Fact]
    public void ResetReturnsToThePreFeedState()
    {
        using var genome = new Genome(StreamSpec);
        const string query = "{\"cmd\":\"cluster\",\"k\":2}";
        string empty = genome.Command(query);

        FeedAll(genome);
        Assert.NotEqual(empty, genome.Command(query));

        genome.Command("{\"cmd\":\"reset\"}");
        Assert.Equal(empty, genome.Command(query));
    }
}
