# A runnable R example: build a genome over a tiny three-symbol universe and
# print the nearest neighbour and the biggest outlier.
#
#   R CMD INSTALL bindings/r
#   Rscript examples/r/genome.R
#
# Every language example runs the same request and prints the same summary — that
# is the cross-language guarantee.
library(wickragenome)

# A price-close genome over three symbols: AAA and BBB sit close together, CCC is
# far away — so AAA's nearest neighbour is BBB and CCC leads the anomaly ranking.
spec <- paste0(
  '{"features":[{"kind":"price","field":"close"}],',
  '"symbols":["AAA","BBB","CCC"],"normalize":"z_score","metric":"euclid","seed":24333}'
)

build_cmd <- paste0(
  '{"cmd":"build","data":{',
  '"AAA":[{"time":0,"open":1,"high":1,"low":1,"close":1,"volume":0}],',
  '"BBB":[{"time":0,"open":2,"high":2,"low":2,"close":2,"volume":0}],',
  '"CCC":[{"time":0,"open":100,"high":100,"low":100,"close":100,"volume":0}]}}'
)

# The first '"symbol":"X"' in a response — the nearest neighbour / top outlier.
first_symbol <- function(json) {
  m <- regmatches(json, regexpr('"symbol":"[^"]+"', json))
  sub('"symbol":"', "", sub('"$', "", m))
}

genome <- wkgenome_new(spec)
wkgenome_command(genome, build_cmd)

similar <- wkgenome_command(genome, '{"cmd":"similar","symbol":"AAA","k":2}')
anomaly <- wkgenome_command(genome, '{"cmd":"anomaly"}')

cat(sprintf("wickra-genome %s\n", wkgenome_version()))
cat(sprintf("AAA nearest: %s\n", first_symbol(similar)))
cat(sprintf("top anomaly: %s\n", first_symbol(anomaly)))
