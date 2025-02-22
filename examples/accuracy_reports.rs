/*
 * Copyright © 2020 Peter M. Stahl pemistahl@gmail.com
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either expressed or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use fraction::{Decimal, Zero};
use include_dir::Dir;
use indoc::formatdoc;
use itertools::Itertools;
use lingua::{Language, LanguageDetectorBuilder};
use lingua_afrikaans_language_model::AFRIKAANS_TESTDATA_DIRECTORY;
use lingua_albanian_language_model::ALBANIAN_TESTDATA_DIRECTORY;
use lingua_arabic_language_model::ARABIC_TESTDATA_DIRECTORY;
use lingua_armenian_language_model::ARMENIAN_TESTDATA_DIRECTORY;
use lingua_azerbaijani_language_model::AZERBAIJANI_TESTDATA_DIRECTORY;
use lingua_basque_language_model::BASQUE_TESTDATA_DIRECTORY;
use lingua_belarusian_language_model::BELARUSIAN_TESTDATA_DIRECTORY;
use lingua_bengali_language_model::BENGALI_TESTDATA_DIRECTORY;
use lingua_bokmal_language_model::BOKMAL_TESTDATA_DIRECTORY;
use lingua_bosnian_language_model::BOSNIAN_TESTDATA_DIRECTORY;
use lingua_bulgarian_language_model::BULGARIAN_TESTDATA_DIRECTORY;
use lingua_catalan_language_model::CATALAN_TESTDATA_DIRECTORY;
use lingua_chinese_language_model::CHINESE_TESTDATA_DIRECTORY;
use lingua_croatian_language_model::CROATIAN_TESTDATA_DIRECTORY;
use lingua_czech_language_model::CZECH_TESTDATA_DIRECTORY;
use lingua_danish_language_model::DANISH_TESTDATA_DIRECTORY;
use lingua_dutch_language_model::DUTCH_TESTDATA_DIRECTORY;
use lingua_english_language_model::ENGLISH_TESTDATA_DIRECTORY;
use lingua_esperanto_language_model::ESPERANTO_TESTDATA_DIRECTORY;
use lingua_estonian_language_model::ESTONIAN_TESTDATA_DIRECTORY;
use lingua_finnish_language_model::FINNISH_TESTDATA_DIRECTORY;
use lingua_french_language_model::FRENCH_TESTDATA_DIRECTORY;
use lingua_ganda_language_model::GANDA_TESTDATA_DIRECTORY;
use lingua_georgian_language_model::GEORGIAN_TESTDATA_DIRECTORY;
use lingua_german_language_model::GERMAN_TESTDATA_DIRECTORY;
use lingua_greek_language_model::GREEK_TESTDATA_DIRECTORY;
use lingua_gujarati_language_model::GUJARATI_TESTDATA_DIRECTORY;
use lingua_hebrew_language_model::HEBREW_TESTDATA_DIRECTORY;
use lingua_hindi_language_model::HINDI_TESTDATA_DIRECTORY;
use lingua_hungarian_language_model::HUNGARIAN_TESTDATA_DIRECTORY;
use lingua_icelandic_language_model::ICELANDIC_TESTDATA_DIRECTORY;
use lingua_indonesian_language_model::INDONESIAN_TESTDATA_DIRECTORY;
use lingua_irish_language_model::IRISH_TESTDATA_DIRECTORY;
use lingua_italian_language_model::ITALIAN_TESTDATA_DIRECTORY;
use lingua_japanese_language_model::JAPANESE_TESTDATA_DIRECTORY;
use lingua_kazakh_language_model::KAZAKH_TESTDATA_DIRECTORY;
use lingua_korean_language_model::KOREAN_TESTDATA_DIRECTORY;
use lingua_latin_language_model::LATIN_TESTDATA_DIRECTORY;
use lingua_latvian_language_model::LATVIAN_TESTDATA_DIRECTORY;
use lingua_lithuanian_language_model::LITHUANIAN_TESTDATA_DIRECTORY;
use lingua_macedonian_language_model::MACEDONIAN_TESTDATA_DIRECTORY;
use lingua_malay_language_model::MALAY_TESTDATA_DIRECTORY;
use lingua_maori_language_model::MAORI_TESTDATA_DIRECTORY;
use lingua_marathi_language_model::MARATHI_TESTDATA_DIRECTORY;
use lingua_mongolian_language_model::MONGOLIAN_TESTDATA_DIRECTORY;
use lingua_nynorsk_language_model::NYNORSK_TESTDATA_DIRECTORY;
use lingua_persian_language_model::PERSIAN_TESTDATA_DIRECTORY;
use lingua_polish_language_model::POLISH_TESTDATA_DIRECTORY;
use lingua_portuguese_language_model::PORTUGUESE_TESTDATA_DIRECTORY;
use lingua_punjabi_language_model::PUNJABI_TESTDATA_DIRECTORY;
use lingua_romanian_language_model::ROMANIAN_TESTDATA_DIRECTORY;
use lingua_russian_language_model::RUSSIAN_TESTDATA_DIRECTORY;
use lingua_serbian_language_model::SERBIAN_TESTDATA_DIRECTORY;
use lingua_shona_language_model::SHONA_TESTDATA_DIRECTORY;
use lingua_slovak_language_model::SLOVAK_TESTDATA_DIRECTORY;
use lingua_slovene_language_model::SLOVENE_TESTDATA_DIRECTORY;
use lingua_somali_language_model::SOMALI_TESTDATA_DIRECTORY;
use lingua_sotho_language_model::SOTHO_TESTDATA_DIRECTORY;
use lingua_spanish_language_model::SPANISH_TESTDATA_DIRECTORY;
use lingua_swahili_language_model::SWAHILI_TESTDATA_DIRECTORY;
use lingua_swedish_language_model::SWEDISH_TESTDATA_DIRECTORY;
use lingua_tagalog_language_model::TAGALOG_TESTDATA_DIRECTORY;
use lingua_tamil_language_model::TAMIL_TESTDATA_DIRECTORY;
use lingua_telugu_language_model::TELUGU_TESTDATA_DIRECTORY;
use lingua_thai_language_model::THAI_TESTDATA_DIRECTORY;
use lingua_tsonga_language_model::TSONGA_TESTDATA_DIRECTORY;
use lingua_tswana_language_model::TSWANA_TESTDATA_DIRECTORY;
use lingua_turkish_language_model::TURKISH_TESTDATA_DIRECTORY;
use lingua_ukrainian_language_model::UKRAINIAN_TESTDATA_DIRECTORY;
use lingua_urdu_language_model::URDU_TESTDATA_DIRECTORY;
use lingua_vietnamese_language_model::VIETNAMESE_TESTDATA_DIRECTORY;
use lingua_welsh_language_model::WELSH_TESTDATA_DIRECTORY;
use lingua_xhosa_language_model::XHOSA_TESTDATA_DIRECTORY;
use lingua_yoruba_language_model::YORUBA_TESTDATA_DIRECTORY;
use lingua_zulu_language_model::ZULU_TESTDATA_DIRECTORY;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;
use strum::IntoEnumIterator;
use titlecase::titlecase;
use whatlang::{Detector, Lang};

struct DetectorStatistics {
    single_word_statistic: Statistic,
    word_pair_statistic: Statistic,
    sentence_statistic: Statistic,
    average_accuracies: HashMap<Language, Decimal>,
}

impl DetectorStatistics {
    fn new() -> Self {
        Self {
            single_word_statistic: Statistic::new(),
            word_pair_statistic: Statistic::new(),
            sentence_statistic: Statistic::new(),
            average_accuracies: HashMap::new(),
        }
    }

    fn add_single_word_counts(&mut self, language: Option<Language>, single_word: &str) {
        self.single_word_statistic.add_language_count(language);
        self.single_word_statistic.add_entity_count();
        self.single_word_statistic
            .add_entity_length_count(single_word);
    }

    fn add_word_pair_counts(&mut self, language: Option<Language>, word_pair: &str) {
        self.word_pair_statistic.add_language_count(language);
        self.word_pair_statistic.add_entity_count();
        self.word_pair_statistic.add_entity_length_count(word_pair);
    }

    fn add_sentence_counts(&mut self, language: Option<Language>, sentence: &str) {
        self.sentence_statistic.add_language_count(language);
        self.sentence_statistic.add_entity_count();
        self.sentence_statistic.add_entity_length_count(sentence);
    }

    fn compute_accuracy_values(&mut self) {
        self.single_word_statistic.map_counts_to_accuracy_values();
        self.word_pair_statistic.map_counts_to_accuracy_values();
        self.sentence_statistic.map_counts_to_accuracy_values();
    }

    fn create_report_data(&mut self, language: &Language) -> Option<String> {
        let (single_word_accuracy, single_word_report) = self
            .single_word_statistic
            .create_report_data(language, "single words");

        let (word_pair_accuracy, word_pair_report) = self
            .word_pair_statistic
            .create_report_data(language, "word pairs");

        let (sentence_accuracy, sentence_report) = self
            .sentence_statistic
            .create_report_data(language, "sentences");

        let average_accuracy =
            (single_word_accuracy + word_pair_accuracy + sentence_accuracy) / Decimal::from(3);

        self.average_accuracies
            .insert(language.clone(), average_accuracy);

        if average_accuracy.is_zero() {
            return None;
        }

        Some(formatdoc!(
            r#"
            ##### {:?} #####

            >>> Accuracy on average: {}

            {}
            {}
            {}
            "#,
            language,
            format_accuracy(average_accuracy),
            single_word_report,
            word_pair_report,
            sentence_report
        ))
    }

    fn create_aggregated_report_row(&self, language: &Language) -> String {
        let average_accuracy_column = match self.average_accuracies.get(language) {
            Some(&accuracy) => {
                if accuracy > Decimal::zero() {
                    accuracy.to_string()
                } else {
                    "NaN".to_string()
                }
            }
            None => "NaN".to_string(),
        };
        let single_words_accuracy_column = match self
            .single_word_statistic
            .language_accuracies
            .get(&Some(language.clone()))
        {
            Some(accuracy) => accuracy.to_string(),
            None => "NaN".to_string(),
        };
        let word_pairs_accuracy_column = match self
            .word_pair_statistic
            .language_accuracies
            .get(&Some(language.clone()))
        {
            Some(accuracy) => accuracy.to_string(),
            None => "NaN".to_string(),
        };
        let sentences_accuracy_column = match self
            .sentence_statistic
            .language_accuracies
            .get(&Some(language.clone()))
        {
            Some(accuracy) => accuracy.to_string(),
            None => "NaN".to_string(),
        };

        format!(
            "{},{},{},{}",
            average_accuracy_column,
            single_words_accuracy_column,
            word_pairs_accuracy_column,
            sentences_accuracy_column
        )
    }
}

struct Statistic {
    language_counts: HashMap<Option<Language>, u32>,
    language_accuracies: HashMap<Option<Language>, Decimal>,
    entity_count: u32,
    entity_length_count: u32,
}

impl Statistic {
    fn new() -> Self {
        Self {
            language_counts: HashMap::new(),
            language_accuracies: HashMap::new(),
            entity_count: 0,
            entity_length_count: 0,
        }
    }

    fn add_language_count(&mut self, language: Option<Language>) {
        let count = self.language_counts.entry(language).or_insert(0);
        *count += 1;
    }

    fn add_entity_count(&mut self) {
        self.entity_count += 1;
    }

    fn add_entity_length_count(&mut self, entity: &str) {
        self.entity_length_count += entity.len() as u32;
    }

    fn map_counts_to_accuracy_values(&mut self) {
        let sum_of_counts: u32 = self.language_counts.values().sum();
        self.language_accuracies = self
            .language_counts
            .iter()
            .map(|(language, count)| {
                (
                    language.clone(),
                    Decimal::from(*count) / Decimal::from(sum_of_counts) * Decimal::from(100),
                )
            })
            .collect();
    }

    fn create_report_data(&self, language: &Language, description: &str) -> (Decimal, String) {
        let accuracy = *self
            .language_accuracies
            .get(&Some(language.clone()))
            .unwrap_or(&Decimal::zero());

        let average_length =
            ((self.entity_length_count as f64) / (self.entity_count as f64)).round();

        (
            accuracy,
            formatdoc!(
                r#"
                >> Detection of {} {} (average length: {} chars)
                Accuracy: {}
                Erroneously classified as {}
                "#,
                self.entity_count,
                description,
                average_length,
                format_accuracy(accuracy),
                self.format_language_accuracies(language)
            ),
        )
    }

    fn format_language_accuracies(&self, language: &Language) -> String {
        self.language_accuracies
            .iter()
            .filter(|(lang, _)| lang.as_ref() != Some(language))
            .sorted_by(
                |(first_lang, &first_accuracy), (second_lang, &second_accuracy)| {
                    let sorted_by_accuracy = second_accuracy.partial_cmp(&first_accuracy).unwrap();
                    let sorted_by_language = first_lang.partial_cmp(&second_lang).unwrap();
                    sorted_by_accuracy.then(sorted_by_language)
                },
            )
            .map(|(lang, &accuracy)| {
                let formatted_lang = if lang.is_some() {
                    format!("{:?}", lang.as_ref().unwrap())
                } else {
                    "Unknown".to_string()
                };
                format!("{}: {}", formatted_lang, format_accuracy(accuracy))
            })
            .join(", ")
    }
}

fn main() {
    let lingua_detector = LanguageDetectorBuilder::from_all_languages().build();
    let whatlang_detector = Detector::new();

    let accuracy_reports_directory = Path::new("accuracy-reports");
    let lingua_reports_directory = accuracy_reports_directory.join("lingua");
    let whatlang_reports_directory = accuracy_reports_directory.join("whatlang");

    if !lingua_reports_directory.is_dir() {
        fs::create_dir_all(&lingua_reports_directory)
            .expect("Lingua reports directory could not be created");
    }

    if !whatlang_reports_directory.is_dir() {
        fs::create_dir_all(&whatlang_reports_directory)
            .expect("Whatlang reports directory could not be created");
    }

    let aggregated_report_file_path =
        accuracy_reports_directory.join("aggregated-accuracy-values.csv");
    let mut aggregated_report_file =
        fs::File::create(aggregated_report_file_path).expect("CSV file could not be created");
    let aggregated_report_columns = vec![
        "language",
        "average-whatlang",
        "single-words-whatlang",
        "word-pairs-whatlang",
        "sentences-whatlang",
        "average-lingua",
        "single-words-lingua",
        "word-pairs-lingua",
        "sentences-lingua\n",
    ];

    aggregated_report_file
        .write_all(aggregated_report_columns.iter().join(",").as_bytes())
        .expect("CSV header row could not be written");

    let total_language_count = Language::iter().count();

    for (idx, language) in Language::iter().enumerate() {
        println!(
            "Writing reports for {:?}... ({}/{})",
            &language,
            (idx + 1),
            total_language_count
        );

        let single_words = get_file_content("single-words.txt", &language);
        let word_pairs = get_file_content("word-pairs.txt", &language);
        let sentences = get_file_content("sentences.txt", &language);

        let mut lingua_statistics = DetectorStatistics::new();
        let mut whatlang_statistics = DetectorStatistics::new();

        for single_word in single_words {
            let lingua_language = lingua_detector.detect_language_of(single_word);
            lingua_statistics.add_single_word_counts(lingua_language, single_word);

            let whatlang_language =
                map_whatlang_to_lingua(whatlang_detector.detect_lang(single_word));
            whatlang_statistics.add_single_word_counts(whatlang_language, single_word);
        }

        for word_pair in word_pairs {
            let lingua_language = lingua_detector.detect_language_of(word_pair);
            lingua_statistics.add_word_pair_counts(lingua_language, word_pair);

            let whatlang_language =
                map_whatlang_to_lingua(whatlang_detector.detect_lang(word_pair));
            whatlang_statistics.add_word_pair_counts(whatlang_language, word_pair);
        }

        for sentence in sentences {
            let lingua_language = lingua_detector.detect_language_of(sentence);
            lingua_statistics.add_sentence_counts(lingua_language, sentence);

            let whatlang_language = map_whatlang_to_lingua(whatlang_detector.detect_lang(sentence));
            whatlang_statistics.add_sentence_counts(whatlang_language, sentence);
        }

        lingua_statistics.compute_accuracy_values();
        whatlang_statistics.compute_accuracy_values();

        let lingua_report = lingua_statistics.create_report_data(&language);
        let whatlang_report = whatlang_statistics.create_report_data(&language);

        let lingua_aggregated_report_row =
            lingua_statistics.create_aggregated_report_row(&language);
        let whatlang_aggregated_report_row =
            whatlang_statistics.create_aggregated_report_row(&language);
        let total_aggregated_report_row = format!(
            "{:?},{},{}\n",
            &language, whatlang_aggregated_report_row, lingua_aggregated_report_row
        );

        aggregated_report_file
            .write_all(total_aggregated_report_row.as_bytes())
            .expect("CSV data row could not be written");

        let report_file_name = titlecase(&format!("{:?}.txt", &language));
        let lingua_reports_file_path = lingua_reports_directory.join(&report_file_name);
        let whatlang_reports_file_path = whatlang_reports_directory.join(&report_file_name);

        if let Some(report) = lingua_report {
            fs::write(lingua_reports_file_path, report)
                .expect("Lingua reports file could not be written");
        }

        if let Some(report) = whatlang_report {
            fs::write(whatlang_reports_file_path, report)
                .expect("Whatlang reports file could not be written");
        }

        println!("Done\n");
    }

    println!("All accuracy reports written successfully");
}

fn get_file_content<'a>(file_name: &'a str, language: &'a Language) -> Vec<&'a str> {
    let directory = get_test_data_directory(language);
    directory
        .get_file(file_name)
        .unwrap()
        .contents_utf8()
        .unwrap()
        .split("\n")
        .filter(|&line| !line.trim().is_empty())
        .collect_vec()
}

fn format_accuracy(accuracy: Decimal) -> String {
    format!("{:.2}%", accuracy)
}

fn map_whatlang_to_lingua(language: Option<Lang>) -> Option<Language> {
    match language {
        Some(Lang::Afr) => Some(Language::Afrikaans),
        Some(Lang::Ara) => Some(Language::Arabic),
        Some(Lang::Azj) => Some(Language::Azerbaijani),
        Some(Lang::Bel) => Some(Language::Belarusian),
        Some(Lang::Ben) => Some(Language::Bengali),
        Some(Lang::Bul) => Some(Language::Bulgarian),
        Some(Lang::Cat) => Some(Language::Catalan),
        Some(Lang::Ces) => Some(Language::Czech),
        Some(Lang::Cmn) => Some(Language::Chinese),
        Some(Lang::Dan) => Some(Language::Danish),
        Some(Lang::Deu) => Some(Language::German),
        Some(Lang::Ell) => Some(Language::Greek),
        Some(Lang::Eng) => Some(Language::English),
        Some(Lang::Epo) => Some(Language::Esperanto),
        Some(Lang::Est) => Some(Language::Estonian),
        Some(Lang::Fin) => Some(Language::Finnish),
        Some(Lang::Fra) => Some(Language::French),
        Some(Lang::Guj) => Some(Language::Gujarati),
        Some(Lang::Heb) => Some(Language::Hebrew),
        Some(Lang::Hin) => Some(Language::Hindi),
        Some(Lang::Hrv) => Some(Language::Croatian),
        Some(Lang::Hun) => Some(Language::Hungarian),
        Some(Lang::Ind) => Some(Language::Indonesian),
        Some(Lang::Ita) => Some(Language::Italian),
        Some(Lang::Jpn) => Some(Language::Japanese),
        Some(Lang::Kat) => Some(Language::Georgian),
        Some(Lang::Kor) => Some(Language::Korean),
        Some(Lang::Lat) => Some(Language::Latin),
        Some(Lang::Lav) => Some(Language::Latvian),
        Some(Lang::Lit) => Some(Language::Lithuanian),
        Some(Lang::Mar) => Some(Language::Marathi),
        Some(Lang::Mkd) => Some(Language::Macedonian),
        Some(Lang::Nld) => Some(Language::Dutch),
        Some(Lang::Nno) => Some(Language::Nynorsk),
        Some(Lang::Nob) => Some(Language::Bokmal),
        Some(Lang::Pan) => Some(Language::Punjabi),
        Some(Lang::Pes) => Some(Language::Persian),
        Some(Lang::Pol) => Some(Language::Polish),
        Some(Lang::Por) => Some(Language::Portuguese),
        Some(Lang::Ron) => Some(Language::Romanian),
        Some(Lang::Rus) => Some(Language::Russian),
        Some(Lang::Slk) => Some(Language::Slovak),
        Some(Lang::Slv) => Some(Language::Slovene),
        Some(Lang::Sna) => Some(Language::Shona),
        Some(Lang::Som) => Some(Language::Somali),
        Some(Lang::Spa) => Some(Language::Spanish),
        Some(Lang::Srp) => Some(Language::Serbian),
        Some(Lang::Swe) => Some(Language::Swedish),
        Some(Lang::Tam) => Some(Language::Tamil),
        Some(Lang::Tel) => Some(Language::Telugu),
        Some(Lang::Tgl) => Some(Language::Tagalog),
        Some(Lang::Tha) => Some(Language::Thai),
        Some(Lang::Tur) => Some(Language::Turkish),
        Some(Lang::Ukr) => Some(Language::Ukrainian),
        Some(Lang::Urd) => Some(Language::Urdu),
        Some(Lang::Vie) => Some(Language::Vietnamese),
        Some(Lang::Yor) => Some(Language::Yoruba),
        Some(Lang::Zul) => Some(Language::Zulu),
        _ => None,
    }
}

fn get_test_data_directory(language: &Language) -> Dir<'static> {
    match language {
        &Language::Afrikaans => AFRIKAANS_TESTDATA_DIRECTORY,
        &Language::Albanian => ALBANIAN_TESTDATA_DIRECTORY,
        &Language::Arabic => ARABIC_TESTDATA_DIRECTORY,
        &Language::Armenian => ARMENIAN_TESTDATA_DIRECTORY,
        &Language::Azerbaijani => AZERBAIJANI_TESTDATA_DIRECTORY,
        &Language::Basque => BASQUE_TESTDATA_DIRECTORY,
        &Language::Belarusian => BELARUSIAN_TESTDATA_DIRECTORY,
        &Language::Bengali => BENGALI_TESTDATA_DIRECTORY,
        &Language::Bokmal => BOKMAL_TESTDATA_DIRECTORY,
        &Language::Bosnian => BOSNIAN_TESTDATA_DIRECTORY,
        &Language::Bulgarian => BULGARIAN_TESTDATA_DIRECTORY,
        &Language::Catalan => CATALAN_TESTDATA_DIRECTORY,
        &Language::Chinese => CHINESE_TESTDATA_DIRECTORY,
        &Language::Croatian => CROATIAN_TESTDATA_DIRECTORY,
        &Language::Czech => CZECH_TESTDATA_DIRECTORY,
        &Language::Danish => DANISH_TESTDATA_DIRECTORY,
        &Language::Dutch => DUTCH_TESTDATA_DIRECTORY,
        &Language::English => ENGLISH_TESTDATA_DIRECTORY,
        &Language::Esperanto => ESPERANTO_TESTDATA_DIRECTORY,
        &Language::Estonian => ESTONIAN_TESTDATA_DIRECTORY,
        &Language::Finnish => FINNISH_TESTDATA_DIRECTORY,
        &Language::French => FRENCH_TESTDATA_DIRECTORY,
        &Language::Ganda => GANDA_TESTDATA_DIRECTORY,
        &Language::Georgian => GEORGIAN_TESTDATA_DIRECTORY,
        &Language::German => GERMAN_TESTDATA_DIRECTORY,
        &Language::Greek => GREEK_TESTDATA_DIRECTORY,
        &Language::Gujarati => GUJARATI_TESTDATA_DIRECTORY,
        &Language::Hebrew => HEBREW_TESTDATA_DIRECTORY,
        &Language::Hindi => HINDI_TESTDATA_DIRECTORY,
        &Language::Hungarian => HUNGARIAN_TESTDATA_DIRECTORY,
        &Language::Icelandic => ICELANDIC_TESTDATA_DIRECTORY,
        &Language::Indonesian => INDONESIAN_TESTDATA_DIRECTORY,
        &Language::Irish => IRISH_TESTDATA_DIRECTORY,
        &Language::Italian => ITALIAN_TESTDATA_DIRECTORY,
        &Language::Japanese => JAPANESE_TESTDATA_DIRECTORY,
        &Language::Kazakh => KAZAKH_TESTDATA_DIRECTORY,
        &Language::Korean => KOREAN_TESTDATA_DIRECTORY,
        &Language::Latin => LATIN_TESTDATA_DIRECTORY,
        &Language::Latvian => LATVIAN_TESTDATA_DIRECTORY,
        &Language::Lithuanian => LITHUANIAN_TESTDATA_DIRECTORY,
        &Language::Macedonian => MACEDONIAN_TESTDATA_DIRECTORY,
        &Language::Malay => MALAY_TESTDATA_DIRECTORY,
        &Language::Maori => MAORI_TESTDATA_DIRECTORY,
        &Language::Marathi => MARATHI_TESTDATA_DIRECTORY,
        &Language::Mongolian => MONGOLIAN_TESTDATA_DIRECTORY,
        &Language::Nynorsk => NYNORSK_TESTDATA_DIRECTORY,
        &Language::Persian => PERSIAN_TESTDATA_DIRECTORY,
        &Language::Polish => POLISH_TESTDATA_DIRECTORY,
        &Language::Portuguese => PORTUGUESE_TESTDATA_DIRECTORY,
        &Language::Punjabi => PUNJABI_TESTDATA_DIRECTORY,
        &Language::Romanian => ROMANIAN_TESTDATA_DIRECTORY,
        &Language::Russian => RUSSIAN_TESTDATA_DIRECTORY,
        &Language::Serbian => SERBIAN_TESTDATA_DIRECTORY,
        &Language::Shona => SHONA_TESTDATA_DIRECTORY,
        &Language::Slovak => SLOVAK_TESTDATA_DIRECTORY,
        &Language::Slovene => SLOVENE_TESTDATA_DIRECTORY,
        &Language::Somali => SOMALI_TESTDATA_DIRECTORY,
        &Language::Sotho => SOTHO_TESTDATA_DIRECTORY,
        &Language::Spanish => SPANISH_TESTDATA_DIRECTORY,
        &Language::Swahili => SWAHILI_TESTDATA_DIRECTORY,
        &Language::Swedish => SWEDISH_TESTDATA_DIRECTORY,
        &Language::Tagalog => TAGALOG_TESTDATA_DIRECTORY,
        &Language::Tamil => TAMIL_TESTDATA_DIRECTORY,
        &Language::Telugu => TELUGU_TESTDATA_DIRECTORY,
        &Language::Thai => THAI_TESTDATA_DIRECTORY,
        &Language::Tsonga => TSONGA_TESTDATA_DIRECTORY,
        &Language::Tswana => TSWANA_TESTDATA_DIRECTORY,
        &Language::Turkish => TURKISH_TESTDATA_DIRECTORY,
        &Language::Ukrainian => UKRAINIAN_TESTDATA_DIRECTORY,
        &Language::Urdu => URDU_TESTDATA_DIRECTORY,
        &Language::Vietnamese => VIETNAMESE_TESTDATA_DIRECTORY,
        &Language::Welsh => WELSH_TESTDATA_DIRECTORY,
        &Language::Xhosa => XHOSA_TESTDATA_DIRECTORY,
        &Language::Yoruba => YORUBA_TESTDATA_DIRECTORY,
        &Language::Zulu => ZULU_TESTDATA_DIRECTORY,
    }
}
