# Roadmap

Wickra Genome is pre-1.0. The vector core, the ten-language binding surface, the
golden corpus and CI are the foundation; these are the themes that follow.

## Near term

- Additional similarity metrics beyond cosine and Euclidean (correlation,
  Mahalanobis over a fitted covariance).
- Streaming / incremental updates: fold a new bar into every asset's vector and
  re-query without a full rebuild.
- Feature weighting and subspace queries — search over a chosen slice of the
  514-dimensional space.

## Later

- A live market feed via `wickra-exchange`, so the genome answers "what looks like
  X **right now**" over a live cross-section of assets.
- Historical fingerprint presets (the cross-time counterpart to the live
  cross-section) for regime and pattern lookup.
- Approximate nearest-neighbour indexing for very large universes, kept
  deterministic.

Nothing here changes the determinism contract: every published result stays
byte-reproducible from its inputs across all ten languages.
