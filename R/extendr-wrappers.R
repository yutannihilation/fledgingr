# Generated by extendr: Do not edit by hand
#
# This file was created with the following call:
#   .Call("wrap__make_fledgingr_wrappers", use_symbols = TRUE, package_name = "fledgingr")

#' @docType package
#' @usage NULL
#' @useDynLib fledgingr, .registration = TRUE
NULL

tokenize_inner <- function(x, config_file, resource_dir, dictionary_path, mode) .Call(wrap__tokenize_inner, x, config_file, resource_dir, dictionary_path, mode)

