using System.Text.Json;
using Wickra.Genome;
using Xunit;

namespace WickraGenome.Tests;

public class GenomeTests
{
    internal const string Spec =
        "{\"features\":[{\"kind\":\"price\",\"field\":\"close\"}]," +
        "\"symbols\":[\"AAA\",\"BBB\",\"CCC\"],\"normalize\":\"z_score\",\"metric\":\"euclid\",\"seed\":24333}";

    internal const string Data =
        "{\"AAA\":[{\"time\":0,\"open\":1,\"high\":1,\"low\":1,\"close\":1,\"volume\":0}]," +
        "\"BBB\":[{\"time\":0,\"open\":2,\"high\":2,\"low\":2,\"close\":2,\"volume\":0}]," +
        "\"CCC\":[{\"time\":0,\"open\":100,\"high\":100,\"low\":100,\"close\":100,\"volume\":0}]}";

    private static Genome Built()
    {
        var g = new Genome(Spec);
        string ok = g.Command("{\"cmd\":\"build\",\"data\":" + Data + "}");
        Assert.Contains("\"ok\":true", ok);
        return g;
    }

    [Fact]
    public void Version_IsNonEmpty()
    {
        Assert.False(string.IsNullOrEmpty(Genome.Version()));
    }

    [Fact]
    public void Similar_ExcludesSelf()
    {
        using var g = Built();
        JsonElement res = JsonDocument.Parse(g.Command("{\"cmd\":\"similar\",\"symbol\":\"AAA\",\"k\":2}")).RootElement;
        JsonElement neighbors = res.GetProperty("neighbors");
        Assert.Equal(2, neighbors.GetArrayLength());
        Assert.Equal("BBB", neighbors[0].GetProperty("symbol").GetString());
    }

    [Fact]
    public void Anomaly_RanksOutlierFirst()
    {
        using var g = Built();
        JsonElement res = JsonDocument.Parse(g.Command("{\"cmd\":\"anomaly\"}")).RootElement;
        Assert.Equal("CCC", res.GetProperty("anomalies")[0].GetProperty("symbol").GetString());
    }

    [Fact]
    public void InvalidSpec_Throws()
    {
        Assert.Throws<ArgumentException>(() => new Genome("{ not valid json"));
    }
}
