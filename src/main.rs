use std::fs::File;

use std::io::prelude::*;
use tv_language_trainer::subtitle::*;

fn _store_to_file(filename: &str, content: Subtitle) {
    let mut file = File::create(filename).unwrap();
    let _x = file.write_all(serde_json::to_string(&content).unwrap().as_bytes());
    // let _x = file.write_all(content.to_string().as_bytes()).unwrap();
}

fn main() {
    // let content = "Pues que si estaba bien, que si descansaba bien. Y me han pedido ellos";
    // println!("{:?}", SubtitleSection::extract_sentences(content));

    // let mut section = SubtitleSection::new();
    // section.text = String::from(content);
    let test_es =
        Subtitle::from_file("Papel01", "./resources/Subtitles/papel_S01E01_es.srt").unwrap();
    let test_en =
        Subtitle::from_file("Papel01", "./resources/Subtitles/papel_S01E01_en.srt").unwrap();
    for (key, sentences) in test_es.sentences {
        for sentence in sentences {
            println!("{}: {}", key, sentence.sentence);
        }
        for sentences in test_en.sentences.get(&key) {
            for sentence in sentences {
                println!("{}: {}", key, sentence.sentence);
            }
        }
    }
}
