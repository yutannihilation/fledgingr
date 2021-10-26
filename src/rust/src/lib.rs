use std::str::FromStr;

use extendr_api::prelude::*;
use sudachi::analysis::stateful_tokenizer::StatefulTokenizer;
use sudachi::analysis::Mode;
use sudachi::config::Config;
use sudachi::dic::dictionary::JapaneseDictionary;
use sudachi::prelude::MorphemeList;
use sudachi::sentence_detector::NonBreakChecker;
use sudachi::sentence_splitter::{SentenceSplitter, SplitSentences};

#[extendr]
fn fledgingr_inner(x: Strings, path: &str, mode: &str) -> List {
    let path = Some(path.into());
    let config = Config::new(None, None, path).unwrap();
    let dict = JapaneseDictionary::from_cfg(&config).unwrap();

    let mode = Mode::from_str(mode).unwrap();

    let mut tokenizer = StatefulTokenizer::create(&dict, false, mode);
    let checker = NonBreakChecker::new(dict.lexicon());
    let splitter = SentenceSplitter::with_limit(32 * 1024).with_checker(&checker);

    let result: Vec<Strings> = x
        .iter()
        .map(|line| {
            let mut morphemes = MorphemeList::empty(&dict);
            let mut surfaces: Vec<String> = Vec::new();

            for (_, sentence) in splitter.split(line.as_str()) {
                tokenizer.reset().push_str(sentence);
                tokenizer.do_tokenize().expect("Failed to tokenize input");

                morphemes
                    .collect_results(&mut tokenizer)
                    .expect("failed to collect results");

                for m in morphemes.iter() {
                    surfaces.push(m.surface().to_string());
                }
            }

            Strings::from_values(surfaces)
        })
        .collect();

    List::from_values(result)
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod fledgingr;
    fn fledgingr_inner;
}
