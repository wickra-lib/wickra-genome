using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;

namespace Wickra.Genome;

/// <summary>Raw P/Invoke surface for the wickra-genome C ABI.</summary>
internal static partial class Native
{
    internal const string Lib = "wickra_genome";

    /// <summary>
    /// Construct a genome handle from a spec JSON (NUL-terminated UTF-8).
    /// Returns null if the spec is null, not valid UTF-8, or not a valid spec.
    /// </summary>
    [LibraryImport(Lib)]
    [UnmanagedCallConv(CallConvs = [typeof(CallConvCdecl)])]
    internal static partial IntPtr wickra_genome_new(byte[] specUtf8);

    /// <summary>Free a genome handle.</summary>
    [LibraryImport(Lib)]
    [UnmanagedCallConv(CallConvs = [typeof(CallConvCdecl)])]
    internal static partial void wickra_genome_free(IntPtr handle);

    /// <summary>
    /// Apply a command JSON (NUL-terminated UTF-8), writing the response into a
    /// caller-owned buffer. Returns the response length, or a negative error code.
    /// </summary>
    [LibraryImport(Lib)]
    [UnmanagedCallConv(CallConvs = [typeof(CallConvCdecl)])]
    internal static partial int wickra_genome_command(IntPtr handle, byte[] cmdUtf8, byte[]? outBuf, nuint cap);

    /// <summary>The library version as a static NUL-terminated string.</summary>
    [LibraryImport(Lib)]
    [UnmanagedCallConv(CallConvs = [typeof(CallConvCdecl)])]
    internal static partial IntPtr wickra_genome_version();
}
