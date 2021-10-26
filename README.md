
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

You can install the development version of fledgingr:

``` r
devtools::install_github("yutannihilation/fledgingr")
```

## Example

``` r
library(fledgingr)

# Download the dictionary from https://github.com/WorksApplications/SudachiDict
Sys.setenv(SUDACHI_DICT_PATH = "~/Downloads/sudachi-dictionary-20210802/system_full.dic")

x <- c(
  "今日もあなただけにニンジャの真実をお伝えしていきます",
  "高輪ゲートウェイ駅"
)

knitr::kable(tokenize(x))
```

|  id | surface      | dictionary_form | reading_form | normalized_form | part_of_speech1 | part_of_speech2 | part_of_speech3 | part_of_speech4 | inflectional_type | inflectional_form |
|----:|:-------------|:----------------|:-------------|:----------------|:----------------|:----------------|:----------------|:----------------|:------------------|:------------------|
|   0 | 今日         | 今日            | キョウ       | 今日            | 名詞            | 普通名詞        | 副詞可能        | \*              | \*                | \*                |
|   0 | も           | も              | モ           | も              | 助詞            | 係助詞          | \*              | \*              | \*                | \*                |
|   0 | あなた       | あなた          | アナタ       | 貴方            | 代名詞          | \*              | \*              | \*              | \*                | \*                |
|   0 | だけ         | だけ            | ダケ         | だけ            | 助詞            | 副助詞          | \*              | \*              | \*                | \*                |
|   0 | に           | に              | ニ           | に              | 助詞            | 格助詞          | \*              | \*              | \*                | \*                |
|   0 | ニンジャ     | ニンジャ        |              | 忍者            | 名詞            | 普通名詞        | 一般            | \*              | \*                | \*                |
|   0 | の           | の              | ノ           | の              | 助詞            | 格助詞          | \*              | \*              | \*                | \*                |
|   0 | 真実         | 真実            | シンジツ     | 真実            | 名詞            | 普通名詞        | 一般            | \*              | \*                | \*                |
|   0 | を           | を              | ヲ           | を              | 助詞            | 格助詞          | \*              | \*              | \*                | \*                |
|   0 | お           | お              | オ           | 御              | 接頭辞          | \*              | \*              | \*              | \*                | \*                |
|   0 | 伝え         | 伝える          | ツタエ       | 伝える          | 動詞            | 一般            | \*              | \*              | 下一段-ア行       | 連用形-一般       |
|   0 | し           | する            | シ           | 為る            | 動詞            | 非自立可能      | \*              | \*              | サ行変格          | 連用形-一般       |
|   0 | て           | て              | テ           | て              | 助詞            | 接続助詞        | \*              | \*              | \*                | \*                |
|   0 | いき         | いく            | イキ         | 行く            | 動詞            | 非自立可能      | \*              | \*              | 五段-カ行         | 連用形-一般       |
|   0 | ます         | ます            | マス         | ます            | 助動詞          | \*              | \*              | \*              | 助動詞-マス       | 終止形-一般       |
|   1 | 高輪         | 高輪            | タカナワ     | 高輪            | 名詞            | 固有名詞        | 地名            | 一般            | \*                | \*                |
|   1 | ゲートウェイ | ゲートウェイ    |              | ゲートウェー    | 名詞            | 普通名詞        | 一般            | \*              | \*                | \*                |
|   1 | 駅           | 駅              | エキ         | 駅              | 名詞            | 普通名詞        | 一般            | \*              | \*                | \*                |

``` r
# Change mode
tokenize("選挙管理委員会", mode = "A")$surface
#> [1] "選挙" "管理" "委員" "会"
tokenize("選挙管理委員会", mode = "B")$surface
#> [1] "選挙"   "管理"   "委員会"
tokenize("選挙管理委員会", mode = "C")$surface
#> [1] "選挙管理委員会"
```
