use crate::toolbox;
use regex::{Match, Regex};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::rc::Rc;
use std::time::Duration;

lazy_static! {
    // let regex = r#"(?m)^\d{1,4}\s+\n"#;
    static ref SECTION_REGEX: Regex = Regex::new(r#"(?m)^\d{1,4}\s+\n"#).unwrap();
    static ref PARTIAL_START_REGEX: Regex = Regex::new(r#"^(?P<before_sentence>[a-z][^\.!?]*[\.!?]?)"#).unwrap();
    static ref SENTENCE_REGEX: Regex = Regex::new(r#"(?P<sentence>¿?¡?[A-Z][^\.!?]*[\.!?])"#).unwrap();
    static ref PARTIAL_END_REGEX: Regex = Regex::new(r#"¿?¡?[A-Z][^\.!?]*[\.!?]\s?(?P<after_sentence>[A-Z][^\.!?]*$)"#).unwrap();
}
#[derive(Eq, PartialEq, Debug)]
pub struct SentenceExtractionResult {
    pub before: Option<String>,
    pub sentences: Option<Vec<String>>,
    pub after: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SubtitleSection {
    pub id: u64,
    pub from: Duration,
    pub to: Duration,
    pub time_index: u128,
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub struct SubtitleSentence {
    pub time_index: u128,
    pub sentence: String,
}

#[derive(Serialize, Deserialize)]
pub struct Subtitle {
    pub name: String,
    pub content: HashMap<u128, Vec<SubtitleSection>>,
}

impl SentenceExtractionResult {
    pub fn from_string(input: &str) -> SentenceExtractionResult {
        let sentence = Rc::new(String::from(input));
        let before_check = Rc::clone(&sentence);
        let after_check = Rc::clone(&sentence);
        let mut sentence_found = true;
        let before: Option<String>;
        let after: Option<String>;
        let sentences: Option<Vec<String>>;
        //check if we have a before @b
        if before_check.find("@b").is_some() {
            //there can be only one @b
            //lets see until where it goes. Either until @s or until the end
            before = match before_check.find("@s") {
                Some(x) => {
                    // let (x, _) = input.split_at(x);
                    Some(String::from(input[2..x].trim()))
                }
                None => {
                    sentence_found = false;
                    match after_check.find("@a") {
                        Some(x) => Some(String::from(input[2..x].trim())),
                        None => Some(String::from(input.trim())),
                    }
                }
            }
        } else {
            //we don't have a before
            before = None;
        }
        //check if we have one or more sentences @s or we can check sentence_found
        sentences = {
            if sentence_found && sentence.find("@s").is_some() {
                //each @s we add to a vectored
                let mut result = Vec::new();
                let ats: Vec<(usize, &str)> = sentence.match_indices("@s").collect();
                for i in 0..ats.len() - 1 {
                    result.push(String::from(input[ats[i].0 + 2..ats[i + 1].0].trim()));
                }
                //we have on left which goes from the right most @s to the first @a or end of string
                match sentence.find("@a") {
                    Some(x) => result.push(String::from(input[ats[ats.len() - 1].0 + 2..x].trim())),
                    None => result.push(String::from(
                        input[ats[ats.len() - 1].0 + 2..input.len()].trim(),
                    )),
                }
                Some(result)
            } else {
                None
            }
        };
        //check if we have an after @a
        after = match after_check.rfind("@a") {
            Some(x) => Some(String::from(input[x + 2..].trim())),
            None => None,
        };
        SentenceExtractionResult {
            before: before,
            sentences: sentences,
            after: after,
        }
    }
}

impl SubtitleSection {
    pub fn new() -> SubtitleSection {
        SubtitleSection {
            id: 0,
            from: Duration::new(0, 0),
            to: Duration::new(0, 0),
            time_index: 0,
            text: String::new(),
        }
    }

    pub fn from_string(block: String) -> Option<SubtitleSection> {
        let mut lines: Vec<&str> = block.split("\r").collect();
        //first item is id
        let id = lines.get(0).unwrap().parse::<u64>().unwrap();
        //second item is the time
        let (from, to) = toolbox::get_times(lines.get(1).unwrap());
        //3 item to last item is text
        let text = match toolbox::get_text(lines.split_off(2)) {
            Some(t) => String::from(toolbox::special_language_replacements(
                toolbox::clean_content_string(&t).trim(),
            )),
            None => {
                println!("Error for id: {}", id);
                return None;
            }
        };
        if text.is_empty() {
            None
        } else {
            Some(SubtitleSection {
                id: id,
                from: from,
                to: to,
                time_index: from.as_millis(),
                text: text,
            })
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.text.push_str(text);
    }

    pub fn to_string(&self) -> String {
        format!("{}\n", self.text)
    }

    pub fn extract_sentences(contents: &str) -> SentenceExtractionResult {
        //First we check if there is a sentence at all
        // let mut all_sentences: Option<Vec<String>> =None;
        let sentences: Option<Vec<String>>;
        let mut sentence_found = false;
        let mut before: Option<String>;
        let after: Option<String>;
        let mut all_sentences = Vec::new();
        let mut before_sentence: String = String::new();
        let mut after_sentence: String = String::new();
        if SENTENCE_REGEX.is_match(&contents) {
            //if there is at least one sentence we should look for all complete sentences in the contents
            //and add each sentence to our result struct
            for caps in SENTENCE_REGEX.captures_iter(&contents) {
                match caps.name("sentence") {
                    Some(c) => {
                        all_sentences.push(String::from(c.as_str()));
                        sentence_found = true;
                    }
                    None => println!("Sentence: None"),
                };
            }
            sentences = Some(all_sentences);
        } else {
            sentences = None;
        }
        //then we should check if there is an partial sentence at the Beginning
        if PARTIAL_START_REGEX.is_match(&contents) {
            //There should be only one
            //Todo: assert it
            match PARTIAL_START_REGEX.captures(&contents) {
                Some(caps) => {
                    match caps.name("before_sentence") {
                        Some(c) => before_sentence = String::from(c.as_str()),
                        None => println!("Before Sentence: None"),
                    };
                }
                None => println!("No Partial Beginning sentence"),
            };
            before = Some(before_sentence);
        } else {
            before = None;
        };
        //and if there is a partial sentence at the end.
        if PARTIAL_END_REGEX.is_match(&contents) {
            //There should be only one
            //Todo: assert it
            match PARTIAL_END_REGEX.captures(&contents) {
                Some(caps) => {
                    match caps.name("after_sentence") {
                        Some(c) => after_sentence = String::from(c.as_str()),
                        None => println!("After: None"),
                    };
                }
                None => println!("No Partial End sentence"),
            };
            after = Some(after_sentence);
        } else {
            after = None;
        };
        // lets check if we found at least one sentence. If not, then the whole input is a partial sentence
        if !sentence_found {
            //assert that before and after are None
            assert!(before.is_none());
            assert!(after.is_none());
            //we store this whole partial sentence in "before"
            before = Some(String::from(contents));
        }
        SentenceExtractionResult {
            sentences,
            before,
            after,
        }
    }
}
impl fmt::Display for SubtitleSection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}-{}\n{}--{}\n{}",
            self.id,
            self.time_index,
            self.from.as_secs(),
            self.to.as_secs(),
            self.text
        )
    }
}

impl Subtitle {
    pub fn new(name: String) -> Subtitle {
        Subtitle {
            name,
            content: HashMap::new(),
        }
    }
    pub fn from_file(name: &str, filename: &str) -> Option<Subtitle> {
        let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
        let matches: Vec<Match> = SECTION_REGEX.find_iter(&contents).collect();
        let mut sections: HashMap<u128, Vec<SubtitleSection>> = HashMap::new();
        for i in 0..matches.len() - 1 {
            match SubtitleSection::from_string(String::from(
                contents
                    .get(matches[i].start()..matches[i + 1].start() - 1)
                    .unwrap()
                    .trim(),
            )) {
                Some(s) => {
                    match sections.get_mut(&(s.time_index / 1000)) {
                        Some(v) => v.push(s),
                        None => {
                            sections.insert(s.time_index / 1000, vec![s]);
                        }
                    }
                    //
                }
                None => (),
            };
        }
        //we have one section left at the end
        match SubtitleSection::from_string(String::from(
            contents
                .get(matches[matches.len() - 1].start()..)
                .unwrap()
                .trim(),
        )) {
            Some(s) => {
                match sections.get_mut(&(s.time_index / 1000)) {
                    Some(v) => v.push(s),
                    None => {
                        sections.insert(s.time_index / 1000, vec![s]);
                    }
                }
                //
            }
            None => (),
        };
        Some(Subtitle {
            name: String::from(name),
            content: sections,
        })
    }
    pub fn to_string(&self) -> String {
        let text = self.content.iter().fold(String::new(), |acc, x| {
            String::from(format!(
                "{}{}",
                acc,
                x.1.iter()
                    .fold(String::new(), |acc, x| format!("{}{}", acc, x))
            ))
        });
        format!("---------{}---------\n{}", self.name, text)
    }
}
impl fmt::Display for Subtitle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = self.content.iter().fold(String::new(), |acc, x| {
            String::from(format!(
                "{}{}",
                acc,
                x.1.iter()
                    .fold(String::new(), |acc, x| format!("{}{}", acc, x))
            ))
        });
        write!(f, "---------{}---------\n{}", self.name, text)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn internal() {
        assert!(true);
    }
}
