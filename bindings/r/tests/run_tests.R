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


## Streaming equals batch, through the same command boundary.
##
## wickra-genome-core proves this in Rust, but that says nothing about the boundary this
## binding crosses: every check above only ever sends {"cmd":"build"}, so `feed`
## was exercised in no language at all. The ramp below is chosen so a candle
## applied twice would change Sma(3), which is what tells the two paths apart.

stream_spec <- paste0(
  '{"features":[{"kind":"indicator","name":"Sma","params":[3]},',
  '{"kind":"price","field":"close"}],"symbols":["AAA","BBB"],',
  '"normalize":"z_score","metric":"euclid","seed":24333}'
)

stream_candle <- function(ts, close) {
  paste0(
    '{"time":', ts, ',"open":', close, ',"high":', close,
    ',"low":', close, ',"close":', close, ',"volume":1}'
  )
}

stream_bars <- list(AAA = c(10, 20, 30), BBB = c(40, 50, 60))

stream_data <- paste0(
  "{",
  paste(
    vapply(names(stream_bars), function(sym) {
      bars <- stream_bars[[sym]]
      paste0(
        '"', sym, '":[',
        paste(vapply(seq_along(bars), function(i) stream_candle(i, bars[i]), ""),
              collapse = ","),
        "]"
      )
    }, ""),
    collapse = ","
  ),
  "}"
)

feed_all <- function(handle) {
  for (sym in names(stream_bars)) {
    bars <- stream_bars[[sym]]
    for (i in seq_along(bars)) {
      wkgenome_command(handle, paste0(
        '{"cmd":"feed","symbol":"', sym, '","candle":', stream_candle(i, bars[i]), "}"
      ))
    }
  }
}

batch_handle <- wkgenome_new(stream_spec)
wkgenome_command(batch_handle, paste0('{"cmd":"build","data":', stream_data, "}"))

stream_handle <- wkgenome_new(stream_spec)
feed_all(stream_handle)

for (query in c(
  '{"cmd":"vector","symbol":"AAA"}',
  '{"cmd":"similar","symbol":"AAA","k":1}',
  '{"cmd":"cluster","k":2}',
  '{"cmd":"anomaly"}'
)) {
  stopifnot(identical(
    wkgenome_command(batch_handle, query),
    wkgenome_command(stream_handle, query)
  ))
}

## Sma(3) over 10, 20, 30 is 20; fed twice each it would be the mean of 20,
## 30, 30 instead, so this value is what tells the two apart.
vector_out <- wkgenome_command(batch_handle, '{"cmd":"vector","symbol":"AAA"}')
stopifnot(grepl('"values":[20,', vector_out, fixed = TRUE))

## reset returns the genome to its pre-feed state
reset_handle <- wkgenome_new(stream_spec)
empty_cluster <- wkgenome_command(reset_handle, '{"cmd":"cluster","k":2}')
feed_all(reset_handle)
stopifnot(!identical(wkgenome_command(reset_handle, '{"cmd":"cluster","k":2}'), empty_cluster))
wkgenome_command(reset_handle, '{"cmd":"reset"}')
stopifnot(identical(wkgenome_command(reset_handle, '{"cmd":"cluster","k":2}'), empty_cluster))

cat("wickra-genome R tests passed\n")
