package wickra

import (
	"encoding/json"
	"testing"
)

const spec = `{"features":[{"kind":"price","field":"close"}],` +
	`"symbols":["AAA","BBB","CCC"],"normalize":"z_score","metric":"euclid","seed":24333}`

const data = `{"AAA":[{"time":0,"open":1,"high":1,"low":1,"close":1,"volume":0}],` +
	`"BBB":[{"time":0,"open":2,"high":2,"low":2,"close":2,"volume":0}],` +
	`"CCC":[{"time":0,"open":100,"high":100,"low":100,"close":100,"volume":0}]}`

func buildGenome(t *testing.T) *Genome {
	t.Helper()
	g, err := New(spec)
	if err != nil {
		t.Fatal(err)
	}
	if _, err := g.Command(`{"cmd":"build","data":` + data + `}`); err != nil {
		t.Fatal(err)
	}
	return g
}

func TestVersion(t *testing.T) {
	if Version() == "" {
		t.Fatal("empty version")
	}
}

func TestSimilarExcludesSelf(t *testing.T) {
	g := buildGenome(t)
	defer g.Close()
	out, err := g.Command(`{"cmd":"similar","symbol":"AAA","k":2}`)
	if err != nil {
		t.Fatal(err)
	}
	var res struct {
		Neighbors []struct {
			Symbol string `json:"symbol"`
		} `json:"neighbors"`
	}
	if err := json.Unmarshal([]byte(out), &res); err != nil {
		t.Fatal(err)
	}
	if len(res.Neighbors) != 2 {
		t.Fatalf("expected 2 neighbors, got %d", len(res.Neighbors))
	}
	if res.Neighbors[0].Symbol != "BBB" {
		t.Fatalf("expected closest neighbor BBB, got %s", res.Neighbors[0].Symbol)
	}
}

func TestAnomalyRanksOutlierFirst(t *testing.T) {
	g := buildGenome(t)
	defer g.Close()
	out, err := g.Command(`{"cmd":"anomaly"}`)
	if err != nil {
		t.Fatal(err)
	}
	var res struct {
		Anomalies []struct {
			Symbol string `json:"symbol"`
		} `json:"anomalies"`
	}
	if err := json.Unmarshal([]byte(out), &res); err != nil {
		t.Fatal(err)
	}
	if res.Anomalies[0].Symbol != "CCC" {
		t.Fatalf("expected outlier CCC first, got %s", res.Anomalies[0].Symbol)
	}
}

func TestInvalidSpecErrors(t *testing.T) {
	if _, err := New(`{"seed":1}`); err == nil {
		t.Fatal("expected an error for an invalid spec")
	}
}
