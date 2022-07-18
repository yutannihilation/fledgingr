
<!-- README.md is generated from README.Rmd. Please edit that file -->

# fledgingr: R Bindings to [Sudachi.rs](https://github.com/WorksApplications/sudachi.rs)

<!-- badges: start -->

[![R-CMD-check](https://github.com/yutannihilation/fledgingr/actions/workflows/check-full.yaml/badge.svg)](https://github.com/yutannihilation/fledgingr/actions/workflows/check-full.yaml)
[![fledgingr status
badge](https://yutannihilation.r-universe.dev/badges/fledgingr)](https://yutannihilation.r-universe.dev)
<!-- badges: end -->

This is an experimental R package to provide an interface to sudachi.rs,
a Rust implementation of Sudachi. Under the hood, this package is
powered by [extendr framework](https://extendr.github.io/) to bridge
between R and Rust.

## Why fledgingr?

fledging = 巣立ち (sudachi)

## Prior work

-   [uribo/sudachir](https://github.com/uribo/sudachir)

## Installation

You can install the development version of fledgingr from r-universe:

``` r
install.packages("fledgingr",
  repos = c(
    yutannihilation = "https://yutannihilation.r-universe.dev",
    CRAN = "https://cloud.r-project.org"
  )
)
```

## Usages

``` r
library(fledgingr)

x <- c(
  "真実をお伝えしていきます",
  "高輪ゲートウェイ駅"
)

tokenize(x)
#>   id            surface    dictionary_form             reading_form
#> 1  0               真実               真実                 シンジツ
#> 2  0                 を                 を                       ヲ
#> 3  0                 お                 お                       オ
#> 4  0               伝え             伝える                   ツタエ
#> 5  0                 し               する                       シ
#> 6  0                 て                 て                       テ
#> 7  0               いき               いく                     イキ
#> 8  0               ます               ます                     マス
#> 9  1 高輪ゲートウェイ駅 高輪ゲートウェイ駅 タカナワゲートウェイエキ
#>      normalized_form part_of_speech1 part_of_speech2 part_of_speech3
#> 1               真実            名詞        普通名詞            一般
#> 2                 を            助詞          格助詞               *
#> 3                 御          接頭辞               *               *
#> 4             伝える            動詞            一般               *
#> 5               為る            動詞      非自立可能               *
#> 6                 て            助詞        接続助詞               *
#> 7               行く            動詞      非自立可能               *
#> 8               ます          助動詞               *               *
#> 9 高輪ゲートウェイ駅            名詞        固有名詞            一般
#>   part_of_speech4 inflectional_type inflectional_form
#> 1               *                 *                 *
#> 2               *                 *                 *
#> 3               *                 *                 *
#> 4               *       下一段-ア行       連用形-一般
#> 5               *          サ行変格       連用形-一般
#> 6               *                 *                 *
#> 7               *         五段-カ行       連用形-一般
#> 8               *       助動詞-マス       終止形-一般
#> 9               *                 *                 *
```

You can change the mode. By default, `"C"` is chosen.

``` r
tokenize("選挙管理委員会", mode = "A")$surface
#> [1] "選挙" "管理" "委員" "会"
tokenize("選挙管理委員会", mode = "B")$surface
#> [1] "選挙"   "管理"   "委員会"
tokenize("選挙管理委員会", mode = "C")$surface
#> [1] "選挙管理委員会"
```

## Licenses

In addition to depend on sudachi.rs, the fledgingr package copies the
several code from sudachi.rs’ repository, which is licensed under the
Apache-2.0 license.

-   The Rust code here is basically based on [the implementation of the
    CLI
    tool](https://github.com/WorksApplications/sudachi.rs/blob/ad1f15818536a379c668ea48fcebaca2278df38e/sudachi-cli/src/main.rs)
-   The files under `inst/sudachi_resources` are copied from
    [`resources` directory on the
    repository](https://github.com/WorksApplications/sudachi.rs/tree/develop/resources)
