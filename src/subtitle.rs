pub mod subtitle {
    use chrono::NaiveTime;
    use regex::{Match, Regex};
    use std::fmt;
    use std::fs;

    pub enum SectionClass {
        Id,
        Time,
        Text,
        None,
    }
    pub struct SubtitleSection {
        pub id: u64,
        pub from: NaiveTime,
        pub to: NaiveTime,
        pub text: String,
    }

    pub struct Subtitle {
        pub name: String,
        pub content: Vec<SubtitleSection>,
    }

    impl SubtitleSection {
        pub fn new() -> SubtitleSection {
            SubtitleSection {
                id: 0,
                from: NaiveTime::from_hms_milli(0, 0, 0, 0),
                to: NaiveTime::from_hms_milli(0, 0, 0, 0),
                text: String::new(),
            }
        }

        fn clean_content_string(input: &str) -> String {
            //<font color="magenta">se estará preguntando</font>
            let html_font_opening_tag = r#"<font color=".{1,10}">"#;
            let html_font_closing_tag = r#"</font>"#;
            let html_italic_opening_tag = r#"<i>"#;
            let html_italic_closing_tag = r#"</i>"#;
            let description = r#"\(.{1,30}\)"#;
            let regex_expressions: Vec<String> = vec![
                String::from(html_font_opening_tag),
                String::from(html_font_closing_tag),
                String::from(description),
                String::from(html_italic_opening_tag),
                String::from(html_italic_closing_tag),
            ];
            let mut result = String::from(input);
            for expr in regex_expressions {
                let regex = Regex::new(&expr).unwrap();
                result = regex.replace_all(&result, " ").into_owned();
            }
            // println!("{}", result);
            result
        }

        fn get_times(time_line: &str) -> (NaiveTime, NaiveTime) {
            let time_regex = r#"(\d{2}:\d{2}:\d{2}.\d{3}) --> (\d{2}:\d{2}:\d{2}.\d{3})"#;
            let time_parser = "%H:%M:%S%.f";
            let time_regex = Regex::new(time_regex).unwrap();
            let time_line = time_line.replace(",", ".");
            let caps = time_regex.captures(&time_line).unwrap();
            (
                NaiveTime::parse_from_str(caps.get(1).unwrap().as_str(), time_parser).unwrap(),
                NaiveTime::parse_from_str(caps.get(2).unwrap().as_str(), time_parser).unwrap(),
            )
        }

        fn get_text(lines: Vec<&str>) -> Option<String> {
            let mut text = String::new();
            match lines.get(0) {
                Some(line) => {
                    text.push_str(&line.trim());
                }
                None => return None,
            };
            match lines.get(1) {
                Some(line) => {
                    text.push_str(&line.trim());
                }
                None => {
                    // println!("{}", text);
                    return Some(text);
                }
            };
            match lines.get(2) {
                Some(line) => {
                    text.push_str(&line.trim());
                    // println!("{}", text);
                    return Some(text);
                }
                None => {
                    // println!("{}", text);
                    return Some(text);
                }
            };
        }

        pub fn from_string(block: String) -> Option<SubtitleSection> {
            // println!("{}", block);
            let mut lines: Vec<&str> = block.split("\r").collect();
            // println!("{:?}", lines);
            //first item is id
            let id = lines.get(0).unwrap().parse::<u64>().unwrap();
            //second item is the time
            let (from, to) = SubtitleSection::get_times(lines.get(1).unwrap());
            //3 item to last item is text
            // let x = lines.split_off(2);
            // println!("{:?}", x);

            let text = match SubtitleSection::get_text(lines.split_off(2)) {
                Some(t) => String::from(SubtitleSection::clean_content_string(&t).trim()),
                None => {
                    println!("Error for id: {}", id);
                    return None;
                }
            };
            // let text = String::from(
            //     SubtitleSection::clean_content_string(
            //         &SubtitleSection::get_text(lines.split_off(2)).unwrap(),
            //     )
            //     .trim(),
            // );
            // println!("{}", text);
            if text.is_empty() {
                None
            } else {
                Some(SubtitleSection {
                    id: id,
                    from: from,
                    to: to,
                    text: text,
                })
            }
            // SubtitleSection::new()
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
            write!(f, "{}\n{}--{}\n{}", self.id, self.from, self.to, self.text)
        }
    }
    // 303
    // 00:22:01,440 --> 00:22:03,480
    // <font color="green">Eras tú ayer</font>
    // <font color="green">el que decías que estabais mal,</font>
    impl Subtitle {
        pub fn new(name: String) -> Subtitle {
            Subtitle {
                name,
                content: Vec::new(),
            }
        }
        pub fn from_file(name: &str, filename: &str) -> Option<Subtitle> {
            let contents =
                fs::read_to_string(filename).expect("Something went wrong reading the file");
            //we split the file into sections
            let regex = r#"(?m)^\d{1,4}\s+\n"#;
            // let regex = r#"-->"#;
            let regex = Regex::new(&regex).unwrap();
            let matches: Vec<Match> = regex.find_iter(&contents).collect();
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
}
