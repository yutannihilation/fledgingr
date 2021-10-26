#' @export
fledgingr <- function(x, mode = c("A", "B", "C")) {
  mode <- match.arg(mode)

  path <- Sys.getenv("SUDACHI_DICT_PATH")
  if (identical(path, "")) {
    stop("Please specify SUDACHI_DICT_PATH envvar first", call. = FALSE)
  }

  path <- path.expand(path)

  fledgingr_inner(x, path, mode)
}
