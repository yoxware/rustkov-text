use rand::Rng;
use std::{collections::HashMap, fmt};

pub struct TextGenError {
    message: String,
}

impl fmt::Display for TextGenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl fmt::Debug for TextGenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TextGenError {{ message: {} }}", self.message)
    }
}

impl Default for TextGenError {
    fn default() -> Self {
        TextGenError {
            message: "Default TextGenError".to_owned(),
        }
    }
}

pub struct TextGenConfig {
    pub num_sentence_output: u32,
    pub sentences_per_paragraph: u32,
    pub output_file: Option<String>,
    pub overwrite_output_file: bool,
}

impl Default for TextGenConfig {
    fn default() -> Self {
        TextGenConfig {
            num_sentence_output: 12,
            sentences_per_paragraph: 6,
            output_file: Some("rustkov_default_output.txt".to_owned()),
            overwrite_output_file: true,
        }
    }
}

pub trait TextGenerator {
    fn new(
        corpus_base: Vec<String>,
        dictionary: HashMap<String, Vec<String>>,
        rng: rand::rngs::ThreadRng,
    ) -> Self;

    fn add_corpus(&mut self, new_corpus: Vec<String>);

    fn remove_corpus(&mut self, old_corpus: Vec<String>);

    fn generate_text(&self, conf: TextGenConfig) -> Result<String, TextGenError>;
}
