pub mod corpus;
pub mod markov;
mod utils;

use corpus::*;
use markov::*;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub struct StandardGenerator {
    base: Vec<String>,
    dict: HashMap<String, Vec<String>>,
    rngen: rand::rngs::ThreadRng,
    need_reinit: bool,
}

impl TextGenerator for StandardGenerator {
    fn new(
        corpus_base: Vec<String>,
        dictionary: HashMap<String, Vec<String>>,
        rng: rand::rngs::ThreadRng,
    ) -> Self {
        StandardGenerator {
            base: corpus_base,
            dict: dictionary,
            rngen: rng,
            need_reinit: true,
        }
    }

    fn add_corpus(&mut self, new_corpus: Vec<String>) {
        for corp in new_corpus {
            self.base.push(corp);
        }
    }

    fn remove_corpus(&mut self, old_corpus: Vec<String>) {
        for corp in old_corpus {
            self.base.retain(|&vecstr| vecstr != corp);
        }
    }

    fn generate_text(&self, conf: TextGenConfig) -> Result<String, TextGenError> {
        let mut output_str = String::new();
        for i in 0u32..conf.num_sentence_output {
            if (i + 1) % conf.sentences_per_paragraph == 0 {
                output_str.push('\n');
                output_str.push('\n');
            }
        }
        Ok(output_str)
    }
}

impl StandardGenerator {
    fn initialize(&mut self, corpus_conf: CorpusConfig) -> bool {
        let mut open_corp = open_corpus(self.base.clone());
        if open_corp.is_err() {
            return false;
        } else {
            read_corpus(open_corp.unwrap(), corpus_conf, &mut self.dict);
            return true;
        }
    }
}
