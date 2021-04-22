use std::io::{BufReader, Read};
use std::path::Path;
use std::{collections::HashMap, io::BufRead};

// enables specification of how the corpus will be read
#[derive(Debug)]
pub struct CorpusConfig {
    n_gram_size: usize,
    suffix_size: usize,
    filters: Vec<String>,
}

pub fn open_corpus(file_paths: Vec<&str>) -> Result<Vec<std::fs::File>, std::io::Error> {
    let mut corpus_vec: Vec<std::fs::File> = vec![];
    for file in file_paths {
        let path = Path::new(file);
        let corpus = std::fs::File::open(path);
        if corpus.is_err() {
            let err_string: String = format!(
                "Unable to open file {}; are you sure the path is correct?",
                file
            );
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                err_string.as_str(),
            ));
        }
        corpus_vec.push(corpus.unwrap())
    }
    Ok(corpus_vec)
}

// TODO: add support for custom suffix sizes and filtering
pub fn read_corpus(
    corpus_vec: Vec<std::fs::File>,
    corpus_conf: CorpusConfig,
    gen_dictionary: &mut HashMap<String, Vec<String>>,
) {
    for corpus in corpus_vec {
        let mut reader = BufReader::new(corpus);
        let mut line = String::new();
        reader.read_line(&mut line);
        let mut split = line.split(" ");
        let vec: Vec<&str> = split.collect();
        for i in 0..vec.len() {
            let mut n_gram = String::new();
            for j in i..corpus_conf.n_gram_size {
                if (i + corpus_conf.n_gram_size) > vec.len() - 1 {
                    break;
                } else {
                    if j < i + corpus_conf.n_gram_size - 1 {
                        n_gram = n_gram + vec[i] + " ";
                    } else {
                        n_gram = n_gram + vec[i];
                    }
                }
            }
            let suffix = vec[i + corpus_conf.n_gram_size];
            if gen_dictionary.contains_key(&n_gram) {
                let mut old_value = gen_dictionary.get(&n_gram).unwrap().clone();
                old_value.push(suffix.to_owned());
                gen_dictionary.insert(n_gram, old_value);
            } else {
                gen_dictionary.insert(n_gram, vec![suffix.to_owned()]);
            }
        }
    }
}
