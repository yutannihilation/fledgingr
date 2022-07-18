#' Tokenize and Analyze Japanese Texts
#'
#' @param x Text to tokenize.
#' @param mode Unit to split text.
#'   * `"A"`: short
#'   * `"B"`: middle
#'   * `"C"`: long
#'
#' @export
tokenize <- function(x, mode = c("C", "B", "A")) {
  mode <- match.arg(mode)

  config_file <- system.file("sudachi_resources", "sudachi.json", package = "fledgingr", mustWork = TRUE)
  resource_dir <- system.file("sudachi_resources", package = "fledgingr", mustWork = TRUE)

  download_sudachi_dict()
  dictonary_path <- fledgingr_dict_file()

  tokenize_inner(x, config_file, resource_dir, dictonary_path, mode)
}

fledgingr_dict_file <- function(create_dir = FALSE) {
  cache_dir <- rappdirs::user_cache_dir("fledgingr")
  if (create_dir) {
    dir.create(cache_dir, recursive = TRUE, showWarnings = FALSE)
  }

  path.expand(file.path(cache_dir, "system_full.dic"))
}

download_sudachi_dict <- function(overwrite = FALSE) {
  dict_file <- fledgingr_dict_file(create_dir = TRUE)
  if (file.exists(dict_file) && !overwrite) {
    return(invisible(NULL))
  }

  cli::cli_inform("Downloading the full dictionary for Sudachi...")

  tempdir <- withr::local_tempdir()
  temp_zip <- file.path(tempdir, "full.zip")

  curl::curl_download(
    "http://sudachi.s3-website-ap-northeast-1.amazonaws.com/sudachidict/sudachi-dictionary-latest-full.zip",
    destfile = temp_zip
  )

  utils::unzip(temp_zip, exdir = tempdir)

  dic_file <- list.files(tempdir, full.names = TRUE, pattern = "system_full\\.dic$", recursive = TRUE)

  if (length(dic_file) == 0L) {
    cli::cli_abort("{.file system_full.dic} was not found!")
  }
  if (length(dic_file) > 1L) {
    names(dic_file) <- rep_len("i", length(dic_file))
    cli::cli_abort(c("multiple {.file system_full.dic} are found!", dic_file))
  }

  cli::cli_inform("Moving {.file system_full.dic} to {.file {dict_file}}")

  file.rename(dic_file, dict_file)

  cli::cli_inform(c(
    "Done.",
    i = "Please visit {.url https://github.com/WorksApplications/SudachiDict} and review the license and the terms of use"
  ))
}
