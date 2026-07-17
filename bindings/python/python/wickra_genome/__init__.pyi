"""Type stubs for the wickra_genome package."""

__version__: str

class Genome:
    """A market genome driven by JSON commands."""

    def __init__(self, spec_json: str) -> None:
        """Construct a genome handle from a spec JSON.

        Raises ``ValueError`` on an invalid spec (an empty ``features`` or
        ``symbols``, or an unknown indicator).
        """
        ...

    def command(self, cmd_json: str) -> str:
        """Apply a command JSON and return the response JSON.

        Domain errors (a bad command, an unknown symbol) come back in-band as
        ``{"ok": false, "error": ...}``; this method does not raise.
        """
        ...

    @staticmethod
    def version() -> str:
        """The library version."""
        ...
