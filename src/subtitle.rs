use crate::toolbox;
use regex::{Match, Regex};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::time::Duration;

lazy_static! {
    // let regex = r#"(?m)^\d{1,4}\s+\n"#;
    static ref SECTION_REGEX: Regex = Regex::new(r#"(?m)^\d{1,4}\s+\n"#).unwrap();
}

#[derive(Serialize, Deserialize)]
pub struct SubtitleSection {
    pub id: u64,
    pub from: Duration,
    pub to: Duration,
    pub time_index: u64,
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub struct Subtitle {
    pub name: String,
    pub content: Vec<SubtitleSection>,
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
            Some(t) => String::from(toolbox::clean_content_string(&t).trim()),
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
                time_index: from.as_secs(),
                text: text,
            })
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.text.push_str(text);
    }

    pub fn to_string(&self) -> String {
        // format!("{}\n{}--{}\n{}", self.id, self.from, self.to, self.text)
        format!("{}\n", self.text)
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
            content: Vec::new(),
        }
    }
    pub fn from_file(name: &str, filename: &str) -> Option<Subtitle> {
        let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
        //we split the file into sections
        // let regex = r#"-->"#;
        // lazy_static! {
        //     // let regex = r#"(?m)^\d{1,4}\s+\n"#;
        //     static ref SECTION_REGEX: Regex = Regex::new(r#"(?m)^\d{1,4}\s+\n"#).unwrap();
        // }
        // let regex = Regex::new(&regex).unwrap();
        let matches: Vec<Match> = SECTION_REGEX.find_iter(&contents).collect();
        let mut sections = Vec::new();
        for i in 0..matches.len() - 1 {
            match SubtitleSection::from_string(String::from(
                contents
                    .get(matches[i].start()..matches[i + 1].start() - 1)
                    .unwrap()
                    .trim(),
            )) {
                Some(s) => sections.push(s),
                None => (),
            };
        }
        match SubtitleSection::from_string(String::from(
            contents
                .get(matches[matches.len() - 1].start()..)
                .unwrap()
                .trim(),
        )) {
            Some(s) => sections.push(s),
            None => (),
        };
        Some(Subtitle {
            name: String::from(name),
            content: sections,
        })
    }
    pub fn to_string(&self) -> String {
        let text = self.content.iter().fold(String::new(), |acc, x| {
            String::from(format!("{}{}", acc, x.to_string()))
        });
        format!("---------{}---------\n{}", self.name, text)
    }
}
impl fmt::Display for Subtitle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = self.content.iter().fold(String::new(), |acc, x| {
            String::from(format!("{}{}", acc, x.to_string()))
        });
        write!(f, "---------{}---------\n{}", self.name, text)
    }
}
