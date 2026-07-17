using Wickra.Genome;
using Xunit;

namespace WickraGenome.Tests;

// Determinism: the same commands yield the byte-identical response string. The
// full cross-language golden (asserting the response equals a blessed
// golden/expected file) lands with the golden corpus in P-GEN-4; here we pin the
// core invariant that the results are byte-reproducible, which every binding
// must preserve by forwarding the command string verbatim.
public class GoldenTests
{
    private static string Run(string cmd)
    {
        using var g = new Genome(GenomeTests.Spec);
        g.Command("{\"cmd\":\"build\",\"data\":" + GenomeTests.Data + "}");
        return g.Command(cmd);
    }

    [Fact]
    public void SameCommands_SameResponse()
    {
        const string cmd = "{\"cmd\":\"cluster\",\"k\":2}";
        Assert.Equal(Run(cmd), Run(cmd));
    }

    [Fact]
    public void Anomaly_FieldOrderIsSymbolFirst()
    {
        Assert.StartsWith("{\"anomalies\":[{\"symbol\":", Run("{\"cmd\":\"anomaly\"}"));
    }
}
