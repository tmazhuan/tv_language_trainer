use regex::Regex;
use std::time::Duration;

pub fn clean_content_string(input: &str) -> String {
    //<font color="magenta">se estará preguntando</font>
    let html_font_opening_tag = r#"<font color=".{1,10}">"#;
    let html_font_closing_tag = r#"</font>"#;
    let html_italic_opening_tag = r#"<i>"#;
    let html_italic_closing_tag = r#"</i>"#;
    let dash = r#"-"#;
    let quotes = r#"""#;
    let description = r#"\(.{1,30}\)"#;
    let regex_expressions: Vec<String> = vec![
        String::from(html_font_opening_tag),
        String::from(html_font_closing_tag),
        String::from(description),
        String::from(html_italic_opening_tag),
        String::from(html_italic_closing_tag),
        String::from(dash),
        String::from(quotes),
    ];
    let mut result = String::from(input);
    for expr in regex_expressions {
        let regex = Regex::new(&expr).unwrap();
        result = regex.replace_all(&result, "").into_owned();
    }
    // println!("{}", result);
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
            // println!("{}", text);
            return Some(text);
        }
    };
    text.push(' ');
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

pub fn get_times(time_line: &str) -> (Duration, Duration) {
    let time_regex = r#"(\d{2}):(\d{2}):(\d{2}).(\d{3}) --> (\d{2}):(\d{2}):(\d{2}).(\d{3})"#;
    // let time_parser = "%H:%M:%S%.f";
    let time_regex = Regex::new(time_regex).unwrap();
    // let time_line = time_line.replace(",", ".");
    let caps = time_regex.captures(&time_line).unwrap();
    let from_hour = caps.get(1).unwrap().as_str().parse::<u64>().unwrap();
    let from_minute = caps.get(2).unwrap().as_str().parse::<u64>().unwrap() + from_hour * 60;
    let from_second = caps.get(3).unwrap().as_str().parse::<u64>().unwrap() + from_minute * 60;
    let from_milli = caps.get(4).unwrap().as_str().parse::<u32>().unwrap();
    let to_hour = caps.get(5).unwrap().as_str().parse::<u64>().unwrap();
    let to_minute = caps.get(6).unwrap().as_str().parse::<u64>().unwrap() + to_hour * 60;
    let to_second = caps.get(7).unwrap().as_str().parse::<u64>().unwrap() + to_minute * 60;
    let to_milli = caps.get(8).unwrap().as_str().parse::<u32>().unwrap();
    (
        Duration::new(from_second, from_milli),
        Duration::new(to_second, to_milli),
    )
}

pub fn extract_sentences(_input: String) {} // -> Vec<String> {}
