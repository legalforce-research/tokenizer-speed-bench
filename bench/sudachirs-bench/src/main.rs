use std::io::BufRead;
use std::path::PathBuf;

use sudachi::analysis::stateful_tokenizer::StatefulTokenizer;
use sudachi::config::Config;
use sudachi::dic::dictionary::JapaneseDictionary;
use sudachi::dic::subset::InfoSubset;
use sudachi::prelude::*;

fn main() {
    let config = Config::new(
        None,
        None,
        Some(PathBuf::from(
            "./bench/sudachi-bench/sudachi-dictionary-20210802/system_core.dic",
        )),
    )
    .unwrap();
    let dict = JapaneseDictionary::from_cfg(&config).unwrap();
    let mut tokenizer = StatefulTokenizer::new(&dict, Mode::C);
    tokenizer.set_subset(InfoSubset::empty());
    let mut morphemes = MorphemeList::empty(&dict);
    let lines: Vec<_> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();
    let mut n_words = 0;

    let start = std::time::Instant::now();
    for line in &lines {
        tokenizer.reset().push_str(line);
        tokenizer.do_tokenize().unwrap();
        morphemes.collect_results(&mut tokenizer).unwrap();
        n_words += morphemes.len();
    }
    let duration = start.elapsed();

    println!("Elapsed-sudachi.rs: {} [sec]", duration.as_secs_f64());

    dbg!(n_words);
}
