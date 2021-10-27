#' @export
tokenize <- function(x, mode = c("C", "B", "A")) {
  mode <- match.arg(mode)

  path <- Sys.getenv("SUDACHI_DICT_PATH")
  if (identical(path, "")) {
    stop("Please specify SUDACHI_DICT_PATH envvar first", call. = FALSE)
  }

  path <- path.expand(path)

  tokenize_inner(x, path, mode)
}
