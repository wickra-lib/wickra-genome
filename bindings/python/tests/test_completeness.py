"""The Python surface exposes exactly the documented API."""

import wickra_genome
from wickra_genome import Genome


def test_module_exports() -> None:
    assert set(wickra_genome.__all__) == {"Genome", "__version__"}


def test_genome_methods() -> None:
    for name in ("command", "version"):
        assert hasattr(Genome, name)


def test_version_is_a_string() -> None:
    assert isinstance(wickra_genome.__version__, str)
    assert wickra_genome.__version__
