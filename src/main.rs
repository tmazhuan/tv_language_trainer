use regex::Match;
use regex::Regex;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use tv_language_trainer::subtitle::subtitle::*;

fn test(filename: &str) {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    //we split the file into sections
    let regex = r#"(?m)(^\d{1,4}\s+\n)"#;
    // println!("{}", &contents);
    // let regex = r#"(?sm)^(.*)$"#;
    // let regex = r#"-->"#;
    let regex = Regex::new(&regex).unwrap();
    for cap in regex.captures_iter(&contents) {
        println!("{}", &cap[1]);
    }
}
fn store_to_file(filename: &str, content: Subtitle) {
    let mut file = File::create(filename).unwrap();
    let x = file.write_all(content.to_string().as_bytes()).unwrap();
}

fn main() {
    let filename = "./resources/Subtitles/papel_S01E10.srt";
    let store_file = "./resources/Subtitles/papel_S01E10.txt";
    // let filename = "./resources/test/test.srt";
    // test(filename);
    let casa_del_papel = Subtitle::from_file("casa del Papel", filename);
    store_to_file(store_file, casa_del_papel.unwrap());
    // println!("{}", casa_del_papel.unwrap());
    // let input = r#"303
    // 00:22:01,440 --> 00:22:03,480
    // <font color="green">Eras tú ayer</font>
    // <font color="green">el que decías que estabais mal,</font>
    // 304
    // 00:22:03,560 --> 00:22:05,600
    // <font color="green">que no la aguantas,</font>
    // <font color="green">que tenéis problemas.</font>
    // 305
    // 00:22:05,680 --> 00:22:07,560
    // <font color="cyan">Problemas como cualquier pareja,</font>
    // <font color="cyan">Mónica,</font>"#;
    // // let r = r#"\n+"#;
    // // let regex = Regex::new(r).unwrap();
    // // let items: Vec<&str> = regex.split(input).map(|s| s.trim()).collect();
    // // for i in items {
    // //     println!("--{}__", i);
    // // }
    // let mut subtitles = Subtitle::new(String::from("Test"));
    // let r = "\n";
    // let time_regex = r#"(\d{2}:\d{2}:\d{2},\d{3}) --> (\d{2}:\d{2}:\d{2},\d{3})"#;
    // let time_regex = Regex::new(time_regex).unwrap();
    // //parse_from_str("08:59:60.123", "%H:%M:%S%,f"),
    // let time_parser = "%H:%M:%S%,f";
    // let input = String::from(input);
    // let items: Vec<&str> = input.split(r).map(|s| s.trim()).collect();
    // let mut ss: SubtitleSection = SubtitleSection::new();
    // let mut section_class = SectionClass::None;
    // let mut empty_section = true;
    // let mut text = "";
    // for i in items {
    //     match section_class {
    //         SectionClass::None => {
    //             ss = SubtitleSection::new();
    //             section_class = SectionClass::Id;
    //         }
    //         SectionClass::Id => {
    //             //interpret i as integer and store it in ss.id
    //             ss.id = i.parse::<u64>().unwrap();
    //             //set section_class to Time
    //             section_class = SectionClass::Time;
    //         }
    //         SectionClass::Time => {
    //             let caps = time_regex.captures(i).unwrap();
    //             ss.from =
    //                 NaiveTime::parse_from_str(caps.get(1).unwrap().as_str(), time_parser).unwrap();
    //             ss.to =
    //                 NaiveTime::parse_from_str(caps.get(2).unwrap().as_str(), time_parser).unwrap();
    //             //advance section_class
    //             section_class = SectionClass::Text;
    //         }
    //         SectionClass::Text => {
    //             //check if line is empty.
    //             if i.is_empty() {
    //                 //check if SubtitleSection contains any text
    //                 if !empty_section {
    //                     //we only add it to the subtitle section vector if there is content
    //                     &ss.add_text(&String::from(text));
    //                     subtitles.content.push(ss);
    //                 }
    //                 section_class = SectionClass::None;
    //                 empty_section = true;
    //             } else {
    //                 empty_section = false;
    //                 //--add text to text of subtitle section
    //                 text = &format!("{}{}", text, i);
    //                 //ss.text.push_str(i);
    //             }
    //         }
    //     };
    // }
    // for s in subtitles.content {
    //     println!("{}\n", s);
    // }

    // let r = r#"\n(\d*)\n\s*(\d{2}:\d{2}:\d{2},\d{3}) --> (\d{2}:\d{2}:\d{2},\d{3})\n([.*\n]+)"#;
    // let r = r#"\n(\d*)\n\s*(\d{2}:\d{2}:\d{2},\d{3}) --> (\d{2}:\d{2}:\d{2},\d{3})\n"#;
    // let r = r#"\s(\d{3})\n\s*(\d{2}:\d{2}:\d{2},\d{3}) --> (\d{2}:\d{2}:\d{2},\d{3})\n\s*(.*)\n\s*(.*)(?:\d*)"#;
    // let r = r#"\s(\d{3})\n\s*(\d{2}:\d{2}:\d{2},\d{3}) --> (\d{2}:\d{2}:\d{2},\d{3})(\n\s*(.*)){1,3}(?:\d*)"#;
    // let regex = Regex::new(r).unwrap();
    // for cap in regex.captures_iter(input) {
    //     println!("{:?}\n-------------------------", cap);
    // }
}
