package wickra

// Streaming equals batch, driven through the JSON command boundary.
//
// wickra-genome-core proves this in Rust, but that says nothing about the boundary this
// binding crosses. Every other test here only ever sends {"cmd":"build"}, so
// `feed` was exercised in no language at all: a binding that mis-serialised a
// candle on the feed path had no test to fail.
//
// The ramp is chosen so a candle applied twice would change Sma(3), which is
// what tells the two paths apart.

import (
	"encoding/json"
	"fmt"
	"testing"
)

const streamSpec = `{"features":[{"kind":"indicator","name":"Sma","params":[3]},` +
	`{"kind":"price","field":"close"}],"symbols":["AAA","BBB"],` +
	`"normalize":"z_score","metric":"euclid","seed":24333}`

// A rising ramp per symbol, three bars each.
var streamBars = map[string][]float64{
	"AAA": {10, 20, 30},
	"BBB": {40, 50, 60},
}

func candleJSON(ts int, close float64) string {
	return fmt.Sprintf(
		`{"time":%d,"open":%g,"high":%g,"low":%g,"close":%g,"volume":1}`,
		ts, close, close, close, close)
}

func streamData() string {
	out := `{`
	for i, sym := range []string{"AAA", "BBB"} {
		if i > 0 {
			out += `,`
		}
		out += fmt.Sprintf(`"%s":[`, sym)
		for j, c := range streamBars[sym] {
			if j > 0 {
				out += `,`
			}
			out += candleJSON(j+1, c)
		}
		out += `]`
	}
	return out + `}`
}

func feedAll(t *testing.T, g *Genome) {
	t.Helper()
	// Sorted symbol order, so the fold order matches the batch path's.
	for _, sym := range []string{"AAA", "BBB"} {
		for j, c := range streamBars[sym] {
			cmd := fmt.Sprintf(`{"cmd":"feed","symbol":"%s","candle":%s}`, sym, candleJSON(j+1, c))
			if _, err := g.Command(cmd); err != nil {
				t.Fatalf("feed %s bar %d: %v", sym, j, err)
			}
		}
	}
}

func TestStreamingEqualsBatch(t *testing.T) {
	batch, err := New(streamSpec)
	if err != nil {
		t.Fatalf("new batch genome: %v", err)
	}
	defer batch.Close()
	if _, err := batch.Command(`{"cmd":"build","data":` + streamData() + `}`); err != nil {
		t.Fatalf("build: %v", err)
	}

	streaming, err := New(streamSpec)
	if err != nil {
		t.Fatalf("new streaming genome: %v", err)
	}
	defer streaming.Close()
	feedAll(t, streaming)

	queries := []string{
		`{"cmd":"vector","symbol":"AAA"}`,
		`{"cmd":"similar","symbol":"AAA","k":1}`,
		`{"cmd":"cluster","k":2}`,
		`{"cmd":"anomaly"}`,
	}
	for _, q := range queries {
		want, err := batch.Command(q)
		if err != nil {
			t.Fatalf("batch %s: %v", q, err)
		}
		got, err := streaming.Command(q)
		if err != nil {
			t.Fatalf("streaming %s: %v", q, err)
		}
		if got != want {
			t.Errorf("streaming != batch for %s\n got: %s\nwant: %s", q, got, want)
		}
	}
}

func TestBatchVectorIsTheMeanOfThreeBars(t *testing.T) {
	g, err := New(streamSpec)
	if err != nil {
		t.Fatalf("new genome: %v", err)
	}
	defer g.Close()
	if _, err := g.Command(`{"cmd":"build","data":` + streamData() + `}`); err != nil {
		t.Fatalf("build: %v", err)
	}
	out, err := g.Command(`{"cmd":"vector","symbol":"AAA"}`)
	if err != nil {
		t.Fatalf("vector: %v", err)
	}
	var vector struct {
		Values []float64 `json:"values"`
	}
	if err := json.Unmarshal([]byte(out), &vector); err != nil {
		t.Fatalf("parse vector: %v", err)
	}
	if len(vector.Values) == 0 || vector.Values[0] != 20 {
		t.Errorf("Sma(3) over 10, 20, 30 is 20, got %v", vector.Values)
	}
}

func TestResetReturnsToThePreFeedState(t *testing.T) {
	g, err := New(streamSpec)
	if err != nil {
		t.Fatalf("new genome: %v", err)
	}
	defer g.Close()

	const query = `{"cmd":"cluster","k":2}`
	empty, err := g.Command(query)
	if err != nil {
		t.Fatalf("cluster on an empty genome: %v", err)
	}

	feedAll(t, g)
	fed, err := g.Command(query)
	if err != nil {
		t.Fatalf("cluster after feeding: %v", err)
	}
	if fed == empty {
		t.Fatal("feeding the whole universe changed nothing")
	}

	if _, err := g.Command(`{"cmd":"reset"}`); err != nil {
		t.Fatalf("reset: %v", err)
	}
	after, err := g.Command(query)
	if err != nil {
		t.Fatalf("cluster after reset: %v", err)
	}
	if after != empty {
		t.Errorf("reset did not return the genome to its pre-feed state")
	}
}
