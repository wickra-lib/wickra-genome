"""Wickra Genome — a vector database of the whole market.

Construct a :class:`Genome` from a :class:`GenomeSpec` JSON, drive it with
command JSONs (``build``, ``feed``, ``vector``, ``similar``, ``cluster``,
``anomaly``, ``version``), and read back the response JSON. The same command
protocol crosses every language binding, so this Python front-end drives the
exact same core — and returns byte-identical similarity, clustering and anomaly
results — as the native CLI.
"""

from ._wickra_genome import Genome, __version__

__all__ = ["Genome", "__version__"]
