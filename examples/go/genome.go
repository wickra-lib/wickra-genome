// A runnable Go example: build a genome over a tiny three-symbol universe and
// print the nearest neighbour and the biggest outlier.
//
//	go run examples/go/genome.go
//
// Every language example runs the same request and prints the same summary —
// that is the cross-language guarantee.
package main

import (
	"encoding/json"
	"fmt"

	wickra "github.com/wickra-lib/wickra-genome/bindings/go"
)

// A price-close genome over three symbols: AAA and BBB sit close together, CCC is
// far away — so AAA's nearest neighbour is BBB and CCC leads the anomaly ranking.
const spec = `{"features":[{"kind":"price","field":"close"}],` +
	`"symbols":["AAA","BBB","CCC"],"normalize":"z_score","metric":"euclid","seed":24333}`

const buildCmd = `{"cmd":"build","data":{` +
	`"AAA":[{"time":0,"open":1,"high":1,"low":1,"close":1,"volume":0}],` +
	`"BBB":[{"time":0,"open":2,"high":2,"low":2,"close":2,"volume":0}],` +
	`"CCC":[{"time":0,"open":100,"high":100,"low":100,"close":100,"volume":0}]}}`

func mustCommand(g *wickra.Genome, cmd string) string {
	out, err := g.Command(cmd)
	if err != nil {
		panic(err)
	}
	return out
}

func main() {
	genome, err := wickra.New(spec)
	if err != nil {
		panic(err)
	}
	defer genome.Close()

	mustCommand(genome, buildCmd)

	var similar struct {
		Neighbors []struct {
			Symbol string `json:"symbol"`
		} `json:"neighbors"`
	}
	var anomaly struct {
		Anomalies []struct {
			Symbol string `json:"symbol"`
		} `json:"anomalies"`
	}
	if err := json.Unmarshal([]byte(mustCommand(genome, `{"cmd":"similar","symbol":"AAA","k":2}`)), &similar); err != nil {
		panic(err)
	}
	if err := json.Unmarshal([]byte(mustCommand(genome, `{"cmd":"anomaly"}`)), &anomaly); err != nil {
		panic(err)
	}

	fmt.Printf("wickra-genome %s\n", wickra.Version())
	fmt.Printf("AAA nearest: %s\n", similar.Neighbors[0].Symbol)
	fmt.Printf("top anomaly: %s\n", anomaly.Anomalies[0].Symbol)
}
