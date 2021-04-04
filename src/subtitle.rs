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
    // static ref PARTIAL_START_REGEX: Regex = Regex::new(r#"^(?P<before_sentence>[a-z][^\.!?]*[\.!?]?)"#).unwrap();
    // static ref SENTENCE_REGEX: Regex = Regex::new(r#"(?P<sentence>¿?¡?[A-Z0-9][^\.!?]*[\.!?][\.]{0,2})"#).unwrap();
    // static ref PARTIAL_END_REGEX: Regex = Regex::new(r#"¿?¡?[0-9A-Z][^\.!?]*[\.!?]\s?(?P<after_sentence>[A-Z][^\.!?]*$)"#).unwrap();
    //Partial Regex is to identify if the whole String is a partial sentence. It cannot contain two partial sentences
    //A Partial sentence is also if the whole input is a beginning of a sentence.
    static ref PARTIAL_REGEX: Regex = Regex::new(r#"^(¿?¡?[A-Za-z0-9][^\.!?]*)$"#).unwrap();
    //If we dont have a match for a partial we check if we have a ending of a sentence in the beginning of the string.
    static ref PARTIAL_ENDING_REGEX: Regex = Regex::new(r#"^(?P<sentence_ending>¿?¡?[a-z][^\.!?]*[\.!?]{1}).*$"#).unwrap();
    //If we have a ending sentence at the beginning then we either have either
    // the start of a new partial sentence
    // one or more  complete sentences
    // one or more complete sentences and the start of a new partial sentence
    //so we are going to first look for the start of a new partial sentences
    static ref PARTIAL_BEGINNING_REGEX: Regex = Regex::new(r#"[^\.!?]*[\.!?]\s?(?P<sentence_beginning>[A-Z][^\.!?]*$)"#).unwrap();
    //and we are collecting all complete sentences
    static ref SENTENCE_REGEX: Regex = Regex::new(r#"(?P<sentence>¿?¡?[A-Z0-9][^\.!?]*[\.!?][\.]{0,2})"#).unwrap();



}
#[derive(Eq, PartialEq, Debug)]
pub struct SentenceExtractionResult {
    pub end_of_a_sentence: Option<String>,
    pub sentences: Option<Vec<String>>,
    pub begin_of_a_sentence: Option<String>,
    pub partial: Option<String>,
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
    ///time_index is the time in milliseconds in which the sentence startet
    pub time_index: u128,
    ///the sentence
    pub sentence: String,
}

#[derive(Serialize, Deserialize)]
pub struct Subtitle {
    pub name: String,
    pub sentences: HashMap<u128, Vec<SubtitleSentence>>,
    pub sections: Vec<SubtitleSection>,
}

impl SentenceExtractionResult {
    pub fn from_string(input: &str) -> SentenceExtractionResult {
        let sentence = Rc::new(String::from(input));
        let ending_check = Rc::clone(&sentence);
        let beginning_check = Rc::clone(&sentence);
        let mut sentence_found = true;
        let ending: Option<String>;
        let beginning: Option<String>;
        let sentences: Option<Vec<String>>;
        if input.find("@p").is_some() {
            //we have a partial only sentence
            return SentenceExtractionResult {
                end_of_a_sentence: None,
                sentences: None,
                begin_of_a_sentence: None,
                partial: Some(String::from(input[2..].trim())),
            };
        }
        //check if we have a ending sentence @e
        if ending_check.find("@e").is_some() {
            //there can be only one @e
            //lets see until where it goes. Either until @s or until the end
            ending = match ending_check.find("@s") {
                Some(x) => {
                    // let (x, _) = input.split_at(x);
                    Some(String::from(input[2..x].trim()))
                }
                None => {
                    sentence_found = false;
                    match beginning_check.find("@b") {
                        Some(x) => Some(String::from(input[2..x].trim())),
                        None => Some(String::from(input[2..].trim())),
                    }
                }
            }
        } else {
            //we don't have a before
            ending = None;
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
                match sentence.find("@b") {
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
        beginning = match beginning_check.rfind("@b") {
            Some(x) => Some(String::from(input[x + 2..].trim())),
            None => None,
        };
        SentenceExtractionResult {
            end_of_a_sentence: ending,
            sentences: sentences,
            begin_of_a_sentence: beginning,
            partial: None,
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
        let mut all_sentences = Vec::new();
        let mut ending_of_a_sentence = None;
        let mut beginning_of_a_sentence = None;

        //---------------------
        //Partial Regex is to identify if the whole String is a partial sentence. It cannot contain two partial sentences
        if PARTIAL_REGEX.is_match(&contents) {
            return SentenceExtractionResult {
                end_of_a_sentence: None,
                sentences: None,
                begin_of_a_sentence: None,
                partial: Some(String::from(contents)),
            };
        }
        //If we dont have a match for a partial we check if we have a ending of a sentence in the beginning of the string.
        if PARTIAL_ENDING_REGEX.is_match(&contents) {
            let caps = PARTIAL_ENDING_REGEX.captures(&contents).unwrap(); //we can do that as we know there is a match and only one
            ending_of_a_sentence =
                Some(String::from(caps.name("sentence_ending").unwrap().as_str()));
        }
        //If we have a ending sentence at the beginning then we either have either
        // the start of a new partial sentence
        // one or more  complete sentences
        // one or more complete sentences and the start of a new partial sentence
        //so we are going to first look for the start of a new partial sentences
        if PARTIAL_BEGINNING_REGEX.is_match(&contents) {
            let caps = PARTIAL_BEGINNING_REGEX.captures(&contents).unwrap();
            beginning_of_a_sentence = Some(String::from(
                caps.name("sentence_beginning").unwrap().as_str(),
            ));
        }
        //and then we are collecting all complete sentences
        if SENTENCE_REGEX.is_match(&contents) {
            //if there is at least one sentence we should look for all complete sentences in the contents
            //and add each sentence to our result struct
            for caps in SENTENCE_REGEX.captures_iter(&contents) {
                match caps.name("sentence") {
                    Some(c) => {
                        all_sentences.push(String::from(c.as_str()));
                    }
                    None => println!("Sentence: None"),
                };
            }
            sentences = Some(all_sentences);
        } else {
            sentences = None;
        }
        SentenceExtractionResult {
            sentences: sentences,
            begin_of_a_sentence: beginning_of_a_sentence,
            end_of_a_sentence: ending_of_a_sentence,
            partial: None,
        }
    }
    //------------------------------
    // if SENTENCE_REGEX.is_match(&contents) {
    //     //if there is at least one sentence we should look for all complete sentences in the contents
    //     //and add each sentence to our result struct
    //     for caps in SENTENCE_REGEX.captures_iter(&contents) {
    //         match caps.name("sentence") {
    //             Some(c) => {
    //                 all_sentences.push(String::from(c.as_str()));
    //                 sentence_found = true;
    //             }
    //             None => println!("Sentence: None"),
    //         };
    //     }
    //     sentences = Some(all_sentences);
    // } else {
    //     sentences = None;
    // }
    // //then we should check if there is an partial sentence at the Beginning
    // if PARTIAL_START_REGEX.is_match(&contents) {
    //     //There should be only one
    //     //Todo: assert it
    //     match PARTIAL_START_REGEX.captures(&contents) {
    //         Some(caps) => {
    //             match caps.name("before_sentence") {
    //                 Some(c) => before_sentence = String::from(c.as_str()),
    //                 None => panic!(
    //                     "Shouldn't happen as we named the regex ourselve check fo a match."
    //                 ),
    //             };
    //         }
    //         None => panic!("Shouldn't happen as we initiall check fo a match."),
    //     };
    //     before = Some(before_sentence);
    // } else {
    //     before = None;
    // };
    // //and if there is a partial sentence at the end.
    // if PARTIAL_END_REGEX.is_match(&contents) {
    //     //There should be only one
    //     //Todo: assert it
    //     match PARTIAL_END_REGEX.captures(&contents) {
    //         Some(caps) => {
    //             match caps.name("after_sentence") {
    //                 Some(c) => after_sentence = String::from(c.as_str()),
    //                 None => panic!(
    //                     "Shouldn't happen as we named the regex ourselve check fo a match."
    //                 ),
    //             };
    //         }
    //         None => panic!("Shouldn't happen as we initiall check fo a match."),
    //     };
    //     after = Some(after_sentence);
    // } else {
    //     after = None;
    // };
    // // lets check if we found either a sentence or a before or an after. If not, then the whole input is a partial sentence
    // if before.is_none() && sentences.is_none() && after.is_none() {
    //     //assert that before is None. after we will overwrite anyway. Also sentence should be none
    //     //we store this whole partial sentence in "before"
    //     partial = Some(String::from(contents));
    // }
    // SentenceExtractionResult {
    //     sentences,
    //     before,
    //     after,
    //     partial,
    // }
    // }
}
impl fmt::Display for SubtitleSection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "[\"{}\",\"{}\"],", self.text, self.text)
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
            sentences: HashMap::new(),
            sections: Vec::new(),
        }
    }
    // pub sentences: HashMap<u128, Vec<SubtitleSection>>,
    // pub sections: Vec<Vec<SubtitleSection>>,

    pub fn from_file(name: &str, filename: &str) -> Option<Subtitle> {
        let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
        let matches: Vec<Match> = SECTION_REGEX.find_iter(&contents).collect();
        let mut sections: Vec<SubtitleSection> = Vec::new();
        for i in 0..matches.len() - 1 {
            match SubtitleSection::from_string(String::from(
                contents
                    .get(matches[i].start()..matches[i + 1].start() - 1)
                    .unwrap()
                    .trim(),
            )) {
                Some(s) => {
                    sections.push(s);
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
                sections.push(s);
            }
            None => (),
        };
        let sentences = Subtitle::extract_sentences_from_sections(&sections);
        Some(Subtitle {
            name: String::from(name),
            sections: sections,
            sentences: sentences,
        })
    }

    /// we have a problem with a section containing the end of a sentence and the beginning of a new one.
    /// for example:
    /// "is in the river. But to save him,""
    /// and following sections where the sentence end and a new one starts
    fn extract_sentences_from_sections(
        sections: &Vec<SubtitleSection>,
    ) -> HashMap<u128, Vec<SubtitleSentence>> {
        //result map stores a vector of SubtitleSentences under the key time_index/1000 -> representing the second the sentence startet
        //the time_index in a SubtitleSentence represents the millisecond in which the sentence startet
        let mut result: HashMap<u128, Vec<SubtitleSentence>> = HashMap::new();
        let mut unfinished_sentence: Option<(u128, String)> = None;
        //for each element in sections
        for section in sections {
            // probably we should check partial results first. If we have one we just add it to the unfinished_sentence and
            //--extract sentences from its text
            let extraction_result = SubtitleSection::extract_sentences(&section.text);
            //--if we have a unfinished sentence from the previous section we append a partial or a ending sentence to this unfinished sentence and store it with the previous setion time_index
            match unfinished_sentence {
                Some((time_index, sentence_from_previous_section)) => {
                    //lets check if the current section is a partial
                    match extraction_result.partial {
                        //if it is we just connect the sentence_from_previous_section with this partial and store it
                        //in unfinished_sentence
                        Some(partial) => {
                            unfinished_sentence = Some((
                                time_index,
                                format!("{} {}", sentence_from_previous_section, partial),
                            ));
                            //and we go to the next section
                            continue;
                        }
                        None => {
                            let sentence_to_store = match extraction_result.end_of_a_sentence {
                                //we append before to the previous sentence
                                //and store it
                                Some(ending) => {
                                    format!("{} {}", sentence_from_previous_section, ending)
                                }
                                //we just store  previous sentence, but remark this situation
                                None => {
                                    println!("We had a previous unfinished sentence but no before and no partial\nPrevious sentence: {}",sentence_from_previous_section);
                                    sentence_from_previous_section
                                }
                            }; //match extraction_result.end_of_a_sentence
                               //Store the new sentence in our Hashmap
                            match result.get_mut(&(time_index / 1000)) {
                                Some(v) => {
                                    v.push(SubtitleSentence {
                                        time_index: time_index,
                                        sentence: sentence_to_store,
                                    });
                                }
                                None => {
                                    result.insert(
                                        time_index / 1000,
                                        vec![SubtitleSentence {
                                            time_index: time_index,
                                            sentence: sentence_to_store,
                                        }],
                                    );
                                }
                            }; //match result.get_mut
                               //and set None for our unfinished sentence
                            unfinished_sentence = None;
                        } //match None arm of extraction_result.partial
                    } //match extraction_result.partial
                } //match Some arm of match unfinished_sentence
                None => (), //unfinished_sentence is already None
            }; //unfinished_sentence
               //we don't have an unfinished sentence. lets check if the new section is a partial
            match extraction_result.partial {
                //if it is we just store it
                //in unfinished_sentence
                Some(partial) => {
                    unfinished_sentence = Some((section.time_index, format!("{}", partial)));
                    //and we go to the next section
                    continue;
                }
                None => (), //
            }
            //--we store all sentences @s from this section in our hashmap with the current sections time-index
            match extraction_result.sentences {
                Some(sentences) => {
                    //if we have a sentence and an unfinishe_sentence we store the unfinished sentence and reset it to None
                    //This situation shouldn't really happen
                    match unfinished_sentence {
                        Some((time_index, sentence_from_previous_section)) => {
                            println!("We had a previous unfinished sentence:\n{}\n but also just found a complete sentence:\n {:?}",sentence_from_previous_section,sentences);
                            match result.get_mut(&(time_index / 1000)) {
                                Some(v) => {
                                    v.push(SubtitleSentence {
                                        time_index: time_index,
                                        sentence: sentence_from_previous_section,
                                    });
                                }
                                None => {
                                    result.insert(
                                        time_index / 1000,
                                        vec![SubtitleSentence {
                                            time_index: time_index,
                                            sentence: sentence_from_previous_section,
                                        }],
                                    );
                                }
                            }; //match result.get_mut
                               //unfinished_sentence = None;we overwrite unfinished_sentence at the end
                        }
                        None => (),
                    }
                    for s in sentences {
                        match result.get_mut(&(section.time_index / 1000)) {
                            Some(v) => v.push(SubtitleSentence {
                                time_index: section.time_index,
                                sentence: s,
                            }),
                            None => {
                                result.insert(
                                    section.time_index / 1000,
                                    vec![SubtitleSentence {
                                        time_index: section.time_index,
                                        sentence: s,
                                    }],
                                );
                            }
                        };
                    }
                }
                None => (),
            };
            //--and we remember the @s section for the next iteration
            unfinished_sentence = match extraction_result.begin_of_a_sentence {
                Some(a) => Some((section.time_index, a)),
                None => None,
            };
        } //for section loop
          //lets check if we have a unfinished sentence at the end. We shouldn't have but just in case
          //we add it as a sentence
        match unfinished_sentence {
            Some((time_index, after)) => {
                println!(
                    "CHECK THIS: We have an after but no more sections...after is:{}",
                    after
                );
                match result.get_mut(&(time_index / 1000)) {
                    Some(v) => v.push(SubtitleSentence {
                        time_index: time_index,
                        sentence: after,
                    }),
                    None => {
                        result.insert(
                            time_index / 1000,
                            vec![SubtitleSentence {
                                time_index: time_index,
                                sentence: after,
                            }],
                        );
                    }
                };
            }
            None => (),
        };
        result
    }
    // pub fn from_file_(name: &str, filename: &str) -> Option<Subtitle> {
    //     let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    //     let matches: Vec<Match> = SECTION_REGEX.find_iter(&contents).collect();
    //     let mut sections: HashMap<u128, Vec<SubtitleSection>> = HashMap::new();
    //     for i in 0..matches.len() - 1 {
    //         match SubtitleSection::from_string(String::from(
    //             contents
    //                 .get(matches[i].start()..matches[i + 1].start() - 1)
    //                 .unwrap()
    //                 .trim(),
    //         )) {
    //             Some(s) => {
    //                 match sections.get_mut(&(s.time_index / 1000)) {
    //                     Some(v) => v.push(s),
    //                     None => {
    //                         sections.insert(s.time_index / 1000, vec![s]);
    //                     }
    //                 }
    //                 //
    //             }
    //             None => (),
    //         };
    //     }
    //     //we have one section left at the end
    //     match SubtitleSection::from_string(String::from(
    //         contents
    //             .get(matches[matches.len() - 1].start()..)
    //             .unwrap()
    //             .trim(),
    //     )) {
    //         Some(s) => {
    //             match sections.get_mut(&(s.time_index / 1000)) {
    //                 Some(v) => v.push(s),
    //                 None => {
    //                     sections.insert(s.time_index / 1000, vec![s]);
    //                 }
    //             }
    //             //
    //         }
    //         None => (),
    //     };
    //     Some(Subtitle {
    //         name: String::from(name),
    //         content: sections,
    //     })
    // }

    pub fn to_string(&self) -> String {
        let text = self.sections.iter().fold(String::new(), |acc, x| {
            String::from(format!("{}{}", acc, x))
        });
        format!("---------{}---------\n{}", self.name, text)
    }
}
impl fmt::Display for Subtitle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = self.sections.iter().fold(String::new(), |acc, x| {
            String::from(format!("{}{}", acc, x))
        });
        write!(f, "---------{}---------\n{}", self.name, text)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct SentenceExtractorTestSet {
        pub test_set: Vec<(String, String)>,
    }

    pub fn _store(input: SentenceExtractorTestSet, filename: &str) -> std::io::Result<()> {
        std::fs::write(filename, toml::to_string(&input).unwrap().as_bytes())
    }
    pub fn read_test_set(filename: &str) -> Result<SentenceExtractorTestSet, io::Error> {
        let content = std::fs::read_to_string(&filename)?;
        let testset: SentenceExtractorTestSet = toml::from_str(&content).unwrap();
        Ok(testset)
    }
    #[test]
    fn test_sentences_testset() {
        let testset = read_test_set("./resources/test/sentence_extractor.toml").unwrap();
        // println!("{:?}", testset);
        for (test, result) in testset.test_set {
            assert_eq!(
                SubtitleSection::extract_sentences(&test),
                SentenceExtractionResult::from_string(&result),
                "{}",
                &test
            );
        }
    }
}
