///Enum which describes the language of the contained String
#[derive(PartialEq, Debug)]
pub enum Language {
    English,
    German,
    Spanish,
}

///Reference (Season and Episode) of a TV Series Episode
#[derive(PartialEq, Debug)]
pub enum TvSeriesReference {
    Season(u32),
    Episode(u32),
}

///Frequency of the word in the corresponding language based on wordlists
#[derive(PartialEq, Debug)]
pub enum WordFrequency {
    VeryHigh,
    High,
    Medium,
    Low,
    VeryLow,
    Undefined,
}

///Proficiency Level of the word according to a 5 level Leitner system. LevelOne is the bucket for no knowledge.
#[derive(PartialEq, Debug)]
pub enum ProficiencyLevel {
    LevelOne,
    LevelTwo,
    LevelThree,
    LevelFour,
    LevelFive,
}
///A struct describing the context of appearance of the corresponding word in a movie or tv series
#[derive(Debug)]
pub struct AppearanceReference {
    pub name: String,
    pub is_movie: bool,
    pub tv_series_reference: Option<TvSeriesReference>,
    pub appearance_at_second: u32,
}

///Struct which describes the content of a TV Language Trainer Flash Card
#[derive(Debug)]
pub struct FlashCard {
    pub word: String,
    pub language: Language,
    pub translation: Vec<String>,
    pub translation_language: Option<Language>,
    pub example_sentence: Vec<String>,
    pub used_in: Vec<AppearanceReference>,
    pub frequency: WordFrequency,
    pub proficiency: ProficiencyLevel,
}

impl FlashCard {
    pub fn new(word: String, language: Language) -> FlashCard {
        FlashCard {
            word: word,
            language: language,
            translation: vec![],
            translation_language: None,
            example_sentence: vec![],
            used_in: vec![],
            frequency: WordFrequency::Undefined,
            proficiency: ProficiencyLevel::LevelOne,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mut card = FlashCard::new(String::from("tomar"), Language::Spanish);
        assert_eq!(card.word, String::from("tomar"));
        assert_eq!(card.language, Language::Spanish);
        assert_eq!(card.translation.len(), 0);
        assert!(card.translation_language.is_none());
        assert_eq!(card.example_sentence.len(), 0);
        assert_eq!(card.used_in.len(), 0);
        assert_eq!(card.frequency, WordFrequency::Undefined);
        assert_eq!(card.proficiency, ProficiencyLevel::LevelOne);
        card.translation.push(String::from("trinken"));
        card.translation_language = Some(Language::German);
    }
}
