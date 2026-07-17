package wickra

import (
	"strings"
	"testing"
)

// Determinism: the same commands yield the byte-identical response string. The
// full cross-language golden (asserting the response equals a blessed
// golden/expected file) lands with the golden corpus in P-GEN-4; here we pin the
// core invariant that the results are byte-reproducible, which every binding
// must preserve by forwarding the command string verbatim.

func run(t *testing.T, cmd string) string {
	t.Helper()
	g, err := New(spec)
	if err != nil {
		t.Fatal(err)
	}
	defer g.Close()
	if _, err := g.Command(`{"cmd":"build","data":` + data + `}`); err != nil {
		t.Fatal(err)
	}
	out, err := g.Command(cmd)
	if err != nil {
		t.Fatal(err)
	}
	return out
}

func TestSameCommandsSameResponse(t *testing.T) {
	cmd := `{"cmd":"cluster","k":2}`
	if run(t, cmd) != run(t, cmd) {
		t.Fatal("cluster response is not byte-reproducible")
	}
}

func TestAnomalyFieldOrderIsSymbolFirst(t *testing.T) {
	out := run(t, `{"cmd":"anomaly"}`)
	if !strings.HasPrefix(out, `{"anomalies":[{"symbol":`) {
		t.Fatalf("unexpected field order: %s", out)
	}
}
