using System.Runtime.InteropServices;
using System.Text;

namespace Wickra.Genome;

/// <summary>
/// A market genome driven by JSON commands, built from a spec, over the Wickra C
/// ABI. Construct one from a spec JSON, drive it with command JSON
/// (<c>build</c>, <c>feed</c>, <c>vector</c>, <c>similar</c>, <c>cluster</c>,
/// <c>anomaly</c>, <c>version</c>) and read back the response JSON — the same
/// protocol as the CLI and every other binding.
/// </summary>
public sealed class Genome : IDisposable
{
    private readonly GenomeHandle _handle;

    /// <summary>Build a genome handle from a spec JSON (a non-empty <c>features</c> and <c>symbols</c>).</summary>
    /// <exception cref="ArgumentException">The spec was not a valid genome spec.</exception>
    public Genome(string specJson)
    {
        IntPtr ptr = Native.wickra_genome_new(Utf8(specJson));
        if (ptr == IntPtr.Zero)
        {
            throw new ArgumentException("wickra-genome: invalid spec", nameof(specJson));
        }
        _handle = new GenomeHandle(ptr);
    }

    /// <summary>Apply a command JSON and return the response JSON.</summary>
    /// <remarks>
    /// Uses the C ABI's length-out protocol: a first call learns the length, then
    /// the response is read into a caller-owned buffer. Domain errors (a bad
    /// command, an unknown symbol) come back in-band as <c>{"ok":false,...}</c>
    /// JSON, not as an exception.
    /// </remarks>
    /// <exception cref="InvalidOperationException">A required argument was unusable or a panic was caught.</exception>
    public string Command(string cmdJson)
    {
        ObjectDisposedException.ThrowIf(_handle.IsInvalid, this);

        byte[] cmd = Utf8(cmdJson);
        IntPtr h = _handle.DangerousGetHandle();
        int n = Native.wickra_genome_command(h, cmd, null, 0);
        if (n < 0)
        {
            throw new InvalidOperationException($"wickra-genome: command failed (code {n})");
        }
        var buf = new byte[n + 1];
        Native.wickra_genome_command(h, cmd, buf, (nuint)buf.Length);
        return Encoding.UTF8.GetString(buf, 0, n);
    }

    /// <summary>The library version.</summary>
    public static string Version() =>
        Marshal.PtrToStringUTF8(Native.wickra_genome_version()) ?? string.Empty;

    /// <summary>Free the native genome handle.</summary>
    public void Dispose() => _handle.Dispose();

    /// <summary>Encode a string as NUL-terminated UTF-8 for the C ABI.</summary>
    private static byte[] Utf8(string s)
    {
        int len = Encoding.UTF8.GetByteCount(s);
        var buf = new byte[len + 1];
        Encoding.UTF8.GetBytes(s, 0, s.Length, buf, 0);
        return buf;
    }
}

/// <summary>A safe handle owning a native genome pointer.</summary>
internal sealed class GenomeHandle : SafeHandle
{
    public GenomeHandle(IntPtr handle)
        : base(IntPtr.Zero, ownsHandle: true) => SetHandle(handle);

    public override bool IsInvalid => handle == IntPtr.Zero;

    protected override bool ReleaseHandle()
    {
        Native.wickra_genome_free(handle);
        return true;
    }
}
