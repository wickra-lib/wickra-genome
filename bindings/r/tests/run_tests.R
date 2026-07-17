## Plain-R tests for the wickra-genome R binding (no testthat dependency).
## Mirrors the Rust/Python/Node/Go/C#/Java tests and doubles as the completeness
## guard: it exercises the full public surface (version + new + command).

library(wickragenome)

spec <- paste0(
  '{"features":[{"kind":"price","field":"close"}],',
  '"symbols":["AAA","BBB","CCC"],"normalize":"z_score",',
  '"metric":"euclid","seed":24333}'
)

data <- paste0(
  '{"AAA":[{"time":0,"open":1,"high":1,"low":1,"close":1,"volume":0}],',
  '"BBB":[{"time":0,"open":2,"high":2,"low":2,"close":2,"volume":0}],',
  '"CCC":[{"time":0,"open":100,"high":100,"low":100,"close":100,"volume":0}]}'
)

build_cmd <- function() {
  paste0('{"cmd":"build","data":', data, "}")
}

## version
stopifnot(nzchar(wkgenome_version()))

## build then query: AAA's closest neighbour is BBB, the outlier CCC leads the
## anomaly ranking
genome <- wkgenome_new(spec)
ok <- wkgenome_command(genome, build_cmd())
stopifnot(grepl('"ok":true', ok, fixed = TRUE))

similar <- wkgenome_command(genome, '{"cmd":"similar","symbol":"AAA","k":2}')
stopifnot(startsWith(similar, '{"neighbors":[{"symbol":"BBB"'))

anomaly <- wkgenome_command(genome, '{"cmd":"anomaly"}')
stopifnot(startsWith(anomaly, '{"anomalies":[{"symbol":"CCC"'))

## results are byte-identical across handles (the cross-language golden core)
genome2 <- wkgenome_new(spec)
wkgenome_command(genome2, build_cmd())
cluster1 <- wkgenome_command(genome, '{"cmd":"cluster","k":2}')
cluster2 <- wkgenome_command(genome2, '{"cmd":"cluster","k":2}')
stopifnot(identical(cluster1, cluster2))

## an invalid spec is a hard error at construction
err <- tryCatch(wkgenome_new("{ not valid json"), error = function(e) e)
stopifnot(inherits(err, "error"))

cat("wickra-genome R tests passed\n")
