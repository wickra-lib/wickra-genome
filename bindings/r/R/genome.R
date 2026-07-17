#' The wickra-genome library version.
#' @return A version string.
#' @export
wkgenome_version <- function() {
  .Call(C_wkgenome_version)
}

#' Build a genome handle from a spec JSON.
#' @param spec_json A `GenomeSpec` JSON string (a non-empty `features` and
#'   `symbols`).
#' @return A `wickra_genome` handle (an external pointer).
#' @export
wkgenome_new <- function(spec_json) {
  .Call(C_wkgenome_new, spec_json)
}

#' Apply a command JSON and return the resulting response JSON.
#' @param genome A genome handle from [wkgenome_new()].
#' @param cmd_json A command JSON string (`build`, `feed`, `vector`, `similar`,
#'   `cluster`, `anomaly`, `version`).
#' @return The response as a JSON string.
#' @export
wkgenome_command <- function(genome, cmd_json) {
  .Call(C_wkgenome_command, genome, cmd_json)
}
