
<!-- README.md is generated from README.Rmd. Please edit that file -->

# fledgingr: An R Bindings Of [Sudachi.rs](https://github.com/WorksApplications/sudachi.rs)

<!-- badges: start -->
<!-- badges: end -->

This is an experimental R package to provide an interface to sudachi.rs,
a Rust implementation of Sudachi. Under the hood, this package is
powered by [extendr framework](https://extendr.github.io/) to bridge
between R and Rust.

The Rust code here is based mainly on [this CLI
tool](https://github.com/WorksApplications/sudachi.rs/blob/ad1f15818536a379c668ea48fcebaca2278df38e/sudachi-cli/src/main.rs)
on the sudachi.rs repository, which is licensed under Apache-2.0
License.

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

## Example

``` r
library(fledgingr)

x <- c(
  "今日もあなただけにニンジャの真実をお伝えしていきます",
  "高輪ゲートウェイ駅"
)

tokenize(x)
#>    id            surface    dictionary_form             reading_form
#> 1   0               今日               今日                   キョウ
#> 2   0                 も                 も                       モ
#> 3   0             あなた             あなた                   アナタ
#> 4   0               だけ               だけ                     ダケ
#> 5   0                 に                 に                       ニ
#> 6   0           ニンジャ           ニンジャ                 ニンジャ
#> 7   0                 の                 の                       ノ
#> 8   0               真実               真実                 シンジツ
#> 9   0                 を                 を                       ヲ
#> 10  0                 お                 お                       オ
#> 11  0               伝え             伝える                   ツタエ
#> 12  0                 し               する                       シ
#> 13  0                 て                 て                       テ
#> 14  0               いき               いく                     イキ
#> 15  0               ます               ます                     マス
#> 16  1 高輪ゲートウェイ駅 高輪ゲートウェイ駅 タカナワゲートウェイエキ
#>       normalized_form part_of_speech1 part_of_speech2 part_of_speech3
#> 1                今日            名詞        普通名詞        副詞可能
#> 2                  も            助詞          係助詞               *
#> 3                貴方          代名詞               *               *
#> 4                だけ            助詞          副助詞               *
#> 5                  に            助詞          格助詞               *
#> 6                忍者            名詞        普通名詞            一般
#> 7                  の            助詞          格助詞               *
#> 8                真実            名詞        普通名詞            一般
#> 9                  を            助詞          格助詞               *
#> 10                 御          接頭辞               *               *
#> 11             伝える            動詞            一般               *
#> 12               為る            動詞      非自立可能               *
#> 13                 て            助詞        接続助詞               *
#> 14               行く            動詞      非自立可能               *
#> 15               ます          助動詞               *               *
#> 16 高輪ゲートウェイ駅            名詞        固有名詞            一般
#>    part_of_speech4 inflectional_type inflectional_form
#> 1                *                 *                 *
#> 2                *                 *                 *
#> 3                *                 *                 *
#> 4                *                 *                 *
#> 5                *                 *                 *
#> 6                *                 *                 *
#> 7                *                 *                 *
#> 8                *                 *                 *
#> 9                *                 *                 *
#> 10               *                 *                 *
#> 11               *       下一段-ア行       連用形-一般
#> 12               *          サ行変格       連用形-一般
#> 13               *                 *                 *
#> 14               *         五段-カ行       連用形-一般
#> 15               *       助動詞-マス       終止形-一般
#> 16               *                 *                 *
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
