use regex::Regex;
use std::time::Duration;

lazy_static! {
    static ref CLEAN_REGEX_VEC: Vec<Regex> = vec![
        Regex::new(&String::from(r#"<font color=".{1,10}">"#)).unwrap(),
        Regex::new(&String::from(r#"</font>"#)).unwrap(),
        Regex::new(&String::from(r#"<i>"#)).unwrap(),
        Regex::new(&String::from(r#"</i>"#)).unwrap(),
        Regex::new(&String::from(r#"""#)).unwrap(),
        Regex::new(&String::from(r#"-"#)).unwrap(),
        Regex::new(&String::from(r#"\(.{1,30}\)"#)).unwrap(),
    ];
    static ref TIME_REGEX: Regex =
        Regex::new(r#"(\d{2}):(\d{2}):(\d{2}).(\d{3}) --> (\d{2}):(\d{2}):(\d{2}).(\d{3})"#)
            .unwrap();
    static ref SENTENCE_REGEX: Regex = Regex::new(r#"(¿?¡?[A-Z][^\.!?]*[\.!?])"#).unwrap();
    static ref SPECIAL_LANGUAGE_REGEX: Vec<(Regex, String)> = vec![
        (Regex::new(r#"¿([^,?]+)\?, "#).unwrap(), String::from("")),
        (Regex::new(r#"(\.\.\.)"#).unwrap(), String::from(" - "))
    ];
}

pub fn clean_content_string(input: &str) -> String {
    let mut result = String::from(input);
    for i in 0..CLEAN_REGEX_VEC.len() {
        result = CLEAN_REGEX_VEC
            .get(i)
            .unwrap()
            .replace_all(&result, "")
            .into_owned();
    }
    result
}

pub fn special_language_replacements(input: &str) -> String {
    let mut result = String::from(input);
    for i in 0..SPECIAL_LANGUAGE_REGEX.len() {
        result = String::from(
            SPECIAL_LANGUAGE_REGEX[i]
                .0
                .replace(&result, &SPECIAL_LANGUAGE_REGEX[i].1)
                .into_owned()
                .trim(),
        )
    }
    result
}

pub fn get_text(lines: Vec<&str>) -> Option<String> {
    let mut text = String::new();
    match lines.get(0) {
        Some(line) => {
            text.push_str(&line.trim());
        }
        None => return None,
    };
    text.push(' ');
    match lines.get(1) {
        Some(line) => {
            text.push_str(&line.trim());
        }
        None => {
            return Some(text);
        }
    };
    text.push(' ');
    match lines.get(2) {
        Some(line) => {
            text.push_str(&line.trim());
            return Some(text);
        }
        None => {
            return Some(text);
        }
    };
}
///r#"(\d{2}):(\d{2}):(\d{2}).(\d{3}) --> (\d{2}):(\d{2}):(\d{2}).(\d{3})"#
pub fn get_times(time_line: &str) -> (Duration, Duration) {
    let caps = TIME_REGEX.captures(&time_line).unwrap();
    let from_hour = caps.get(1).unwrap().as_str().parse::<u64>().unwrap();
    let from_minute = caps.get(2).unwrap().as_str().parse::<u64>().unwrap() + from_hour * 60;
    let from_second = caps.get(3).unwrap().as_str().parse::<u64>().unwrap() + from_minute * 60;
    let from_milli = caps.get(4).unwrap().as_str().parse::<u64>().unwrap() + from_second * 1000;
    let to_hour = caps.get(5).unwrap().as_str().parse::<u64>().unwrap();
    let to_minute = caps.get(6).unwrap().as_str().parse::<u64>().unwrap() + to_hour * 60;
    let to_second = caps.get(7).unwrap().as_str().parse::<u64>().unwrap() + to_minute * 60;
    let to_milli = caps.get(8).unwrap().as_str().parse::<u64>().unwrap() + to_second * 1000;
    let d_from = Duration::from_millis(from_milli);
    assert_eq!(from_second, d_from.as_secs());
    let d_to = Duration::from_millis(to_milli);
    assert_eq!(to_second, d_to.as_secs());
    (
        Duration::from_millis(from_milli),
        Duration::from_millis(to_milli),
    )
}

pub fn extract_sentences(input: String) -> Vec<String> {
    SENTENCE_REGEX
        .captures_iter(&input)
        .map(|x| format!("{}", &x[1]))
        .collect()
}
