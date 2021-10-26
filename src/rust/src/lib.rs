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
fn tokenize_inner(x: Strings, path: &str, mode: &str) -> Robj {
    let path = Some(path.into());
    let config = Config::new(None, None, path).unwrap();
    let dict = JapaneseDictionary::from_cfg(&config).unwrap();

    let mode = Mode::from_str(mode).unwrap();

    let mut tokenizer = StatefulTokenizer::create(&dict, false, mode);
    let checker = NonBreakChecker::new(dict.lexicon());
    let splitter = SentenceSplitter::with_limit(32 * 1024).with_checker(&checker);

    // TODO: create a struct
    let capacity = x.len();
    let mut id: Vec<u32> = Vec::with_capacity(capacity);
    let mut surface: Vec<String> = Vec::with_capacity(capacity);
    let mut dictionary_form: Vec<String> = Vec::with_capacity(capacity);
    let mut reading_form: Vec<String> = Vec::with_capacity(capacity);
    let mut normalized_form: Vec<String> = Vec::with_capacity(capacity);
    let mut part_of_speech1: Vec<String> = Vec::with_capacity(capacity);
    let mut part_of_speech2: Vec<String> = Vec::with_capacity(capacity);
    let mut part_of_speech3: Vec<String> = Vec::with_capacity(capacity);
    let mut part_of_speech4: Vec<String> = Vec::with_capacity(capacity);
    let mut inflectional_type: Vec<String> = Vec::with_capacity(capacity);
    let mut inflectional_form: Vec<String> = Vec::with_capacity(capacity);

    for (i, line) in x.iter().enumerate() {
        let mut morphemes = MorphemeList::empty(&dict);

        for (_, sentence) in splitter.split(line.as_str()) {
            tokenizer.reset().push_str(sentence);
            tokenizer.do_tokenize().expect("Failed to tokenize input");

            morphemes
                .collect_results(&mut tokenizer)
                .expect("failed to collect results");

            for m in morphemes.iter() {
                id.push(i as _);
                surface.push(m.surface().to_string());
                dictionary_form.push(m.dictionary_form().to_string());
                reading_form.push(m.reading_form().to_string());
                normalized_form.push(m.normalized_form().to_string());

                let part_of_speech = m.part_of_speech().unwrap();
                part_of_speech1.push(part_of_speech[0].clone());
                part_of_speech2.push(part_of_speech[1].clone());
                part_of_speech3.push(part_of_speech[2].clone());
                part_of_speech4.push(part_of_speech[3].clone());
                inflectional_type.push(part_of_speech[4].clone());
                inflectional_form.push(part_of_speech[5].clone());
            }
        }
    }

    data_frame!(
        id = r!(id),
        surface = r!(surface),
        dictionary_form = r!(dictionary_form),
        reading_form = r!(reading_form),
        normalized_form = r!(normalized_form),
        part_of_speech1 = r!(part_of_speech1),
        part_of_speech2 = r!(part_of_speech2),
        part_of_speech3 = r!(part_of_speech3),
        part_of_speech4 = r!(part_of_speech4),
        inflectional_type = r!(inflectional_type),
        inflectional_form = r!(inflectional_form)
    )
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod fledgingr;
    fn tokenize_inner;
}
