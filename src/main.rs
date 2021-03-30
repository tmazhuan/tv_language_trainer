use regex::Regex;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use tv_language_trainer::subtitle::SentenceExtractionResult;
use tv_language_trainer::subtitle::*;
use tv_language_trainer::toolbox;
#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref PARTIAL_START_REGEX: Regex = Regex::new(r#"^(?P<before_sentence>[a-z][^\.!?]*[\.!?]?)"#).unwrap();
    static ref SENTENCE_REGEX: Regex = Regex::new(r#"(?P<sentence>¿?¡?[A-Z][^\.!?]*[\.!?])"#).unwrap();
    static ref PARTIAL_END_REGEX: Regex = Regex::new(r#"¿?¡?[A-Z][^\.!?]*[\.!?]\s?(?P<after_sentence>[A-Z][^\.!?]*$)"#).unwrap();
    // static ref SENTENCE_REGEX: Regex = Regex::new(r#"(¿?¡?[A-Z][^\.!?]*[\.!?]^,?)"#).unwrap();
}

fn _test(contents: &str) {
    for caps in SENTENCE_REGEX.captures_iter(&contents) {
        println!("{}", caps.name("sentence").unwrap().as_str())
    }
}
fn test_sentence(contents: &str) -> SentenceExtractionResult {
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

fn _store_to_file(filename: &str, content: Subtitle) {
    let mut file = File::create(filename).unwrap();
    let _x = file.write_all(serde_json::to_string(&content).unwrap().as_bytes());
    // let _x = file.write_all(content.to_string().as_bytes()).unwrap();
}

fn main() {
    let x = SentenceExtractionResult::from_string(
        // r#"@bde compras.@s Y, no sé, mira a ver si encuentras algo."#,
        // r#"@bPues eso ya acojona más, porque lo que tienes que hacer,"#,
        r#"@sPues que si estaba bien, que si descansaba bien. @aY me han pedido ellos"#,
    );
    let y = SubtitleSection::extract_sentences(
        r#"Pues que si estaba bien, que si descansaba bien. Y me han pedido ellos"#,
    );
    println!("{:?},{:?},{:?}", x.before, x.sentences, x.after);
    println!("{:?},{:?},{:?}", y.before, y.sentences, y.after);
    assert_eq!(x, y);
    // let sentence: Vec<String> = vec![
    //     toolbox::special_language_replacements(
    //         r#"Pues eso ya acojona más, porque lo que tienes que hacer,"#,
    //     ),
    //     toolbox::special_language_replacements(
    //         r#"de compras. Y, no sé, mira a ver si encuentras algo."#,
    //     ),
    //     toolbox::special_language_replacements(r#"No, yo no llevo gafas."#),
    //     toolbox::special_language_replacements(
    //         r#"Pues que si estaba bien, que si descansaba bien. Y me han pedido ellos"#,
    //     ),
    //     toolbox::special_language_replacements(
    //         r#"de compras. Y, no sé mira a ver si encuentras algo."#,
    //     ),
    //     toolbox::special_language_replacements(
    //         r#"Dilo tú, repítelo. Mírate y repítelo, dilo, dilo."#,
    //     ),
    // ];
    // let assertions = vec![SentenceExtractionResult {
    //     before: None,
    //     sentences: Some(vec![String::from(
    //         "de compras. Y, no sé, mira a ver si encuentras algo.",
    //     )]),
    //     after: None,
    // }];
    // for s in sentence {
    //     println!("Input: {}", s);
    //     let result = test_sentence(&s);
    //     println!(
    //         "before: {:?}\nsentences: {:?}\nafter: {:?}\n------------------",
    //         result.before, result.sentences, result.after
    //     );
    // }
}
