// Package wickra provides idiomatic Go bindings for wickra-genome over its C ABI
// hub: build a Genome from a spec JSON, drive it with command JSON (build, feed,
// vector, similar, cluster, anomaly, version) and read back the response JSON —
// the same protocol as the CLI and every other binding.
//
// The binding links the prebuilt C ABI library, staged per platform under
// ./lib/<goos>_<goarch>/, with the header vendored under ./include.
package wickra

/*
#cgo CFLAGS: -I${SRCDIR}/include
#cgo linux,amd64 LDFLAGS: -L${SRCDIR}/lib/linux_amd64 -lwickra_genome -Wl,-rpath,${SRCDIR}/lib/linux_amd64
#cgo linux,arm64 LDFLAGS: -L${SRCDIR}/lib/linux_arm64 -lwickra_genome -Wl,-rpath,${SRCDIR}/lib/linux_arm64
#cgo darwin,amd64 LDFLAGS: -L${SRCDIR}/lib/darwin_amd64 -lwickra_genome -Wl,-rpath,${SRCDIR}/lib/darwin_amd64
#cgo darwin,arm64 LDFLAGS: -L${SRCDIR}/lib/darwin_arm64 -lwickra_genome -Wl,-rpath,${SRCDIR}/lib/darwin_arm64
#cgo windows,amd64 LDFLAGS: -L${SRCDIR}/lib/windows_amd64 -l:wickra_genome.dll
#cgo windows,arm64 LDFLAGS: -L${SRCDIR}/lib/windows_arm64 -l:wickra_genome.dll
#include <stdlib.h>
#include "wickra_genome.h"
*/
import "C"

import (
	"fmt"
	"runtime"
	"unsafe"
)

// Genome is a market genome driven by JSON commands, built from a spec.
type Genome struct {
	handle *C.WickraGenome
}

// New builds a genome handle from a spec JSON string (a non-empty features and
// symbols). It returns an error if the spec is null, not valid UTF-8, or not a
// valid spec. Call Close when done (a finalizer also frees it, but explicit
// Close is preferred).
func New(specJSON string) (*Genome, error) {
	cspec := C.CString(specJSON)
	defer C.free(unsafe.Pointer(cspec))

	handle := C.wickra_genome_new(cspec)
	if handle == nil {
		return nil, fmt.Errorf("wickra-genome: invalid spec")
	}
	g := &Genome{handle: handle}
	runtime.SetFinalizer(g, (*Genome).Close)
	return g, nil
}

// Command applies a command JSON and returns the response JSON. It uses the C
// ABI's length-out protocol: a first call learns the length, then the response
// is read into a caller-owned buffer. Domain errors come back in-band as
// {"ok":false,"error":...}.
func (g *Genome) Command(cmdJSON string) (string, error) {
	ccmd := C.CString(cmdJSON)
	defer C.free(unsafe.Pointer(ccmd))

	n := C.wickra_genome_command(g.handle, ccmd, nil, 0)
	if n < 0 {
		return "", fmt.Errorf("wickra-genome: command failed (code %d)", int(n))
	}
	buf := make([]byte, int(n)+1)
	C.wickra_genome_command(
		g.handle,
		ccmd,
		(*C.char)(unsafe.Pointer(&buf[0])),
		C.uintptr_t(len(buf)),
	)
	return string(buf[:n]), nil
}

// Close frees the genome handle. Safe to call more than once.
func (g *Genome) Close() {
	if g.handle != nil {
		C.wickra_genome_free(g.handle)
		g.handle = nil
	}
	runtime.SetFinalizer(g, nil)
}

// Version returns the library version.
func Version() string {
	return C.GoString(C.wickra_genome_version())
}
