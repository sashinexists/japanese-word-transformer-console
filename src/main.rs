extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use serde::Deserializer;
use serde_json::json;
use serde_json::Value as JsonValue;

use regex::Regex;
use std::{
    fmt, fs, io,
    str::{self, FromStr},
};

#[derive(Serialize, Deserialize, Debug)]
struct Joshi {
    name: String,
    function: String,
    joins_with: Vec<WordType>,
    godan_stem: GodanStem,
    ichidan_form: String,
    word_type: WordType,
}

fn main() {
    //let example_verbs = vec!["買う", "聞く","話す","持つ","死ぬ","飛ぶ","飲む","取る"];
    //categorise_verbs(example_verbs);

    const JOSHI_DATA_FILE_PATH: &str = "src/joshi.json";

    let joshi_data =
        fs::read_to_string(JOSHI_DATA_FILE_PATH).expect("Something went wrong reading the file");

    //println!("With text:\n{}", joshi_data);

    let json_str = r#"{
        "name": "ます",
        "function": "Makes it formal",
        "joins_with": [
            "Doushi"
        ],
        "godan_stem": "i",
        "ichidan_form": "N/A",
        "word_type": "Doushi"
    }"#;

    let joshi_list: Vec<Joshi> = serde_json::from_str(joshi_data.as_str()).unwrap();
    println!("{:#?}", joshi_list[3]);

    //println!("Is the joshi string valid? {}", validate_joshi_json(json_str));

    /*
    let res = serde_json::from_str(json_str);

    let mut name = "borked";
    let mut function = "borked";
    let mut word_type = "borked";
    if res.is_ok() {
        let p: JsonValue = res.unwrap();
        let name_res = p["name"].as_str();
        if name_res.is_some() {
            name = name_res.unwrap();
        }
        let word_type_res = p["word_type"].as_str();
        if word_type_res.is_some() {
            word_type = word_type_res.unwrap();
        }
        let function_res = p["function"].as_str();
        if function_res.is_some() {
            function = function_res.unwrap();
        }

        println!(
            "The {} joshi is a {} that {}",
            name, word_type, function
        );
    } else {
        "Sorry could not parse JSON";
    }*/

    //verb_categoriser_loop();
}

fn verb_categoriser_loop() {
    let mut input = String::new();
    while input != "出る" {
        input.clear();
        println!("動詞を入力下さい　(Please enter a verb) | Alternatively, type 出る to exit");
        io::stdin().read_line(&mut input).unwrap();
        input.pop();
        if input != "出る" {
            categorise_verbs(vec![input.as_str()]);
        }
    }
    println!("さようなら");
}

#[derive(Serialize, Deserialize)]
enum VerbGroup {
    Ichidan,
    Godan,
    Kurusuru,
}

#[derive(Serialize, Deserialize, Debug)]
enum WordType {
    Meishi,
    Doushi,
    Keiyoushi,
    Joshi,
}

#[derive(Serialize, Deserialize, Debug)]
enum GodanStem {
    A,
    I,
    U,
    E,
    O,
}

impl fmt::Display for VerbGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VerbGroup::Ichidan => write!(f, "Ichidan"),
            VerbGroup::Godan => write!(f, "Godan"),
            VerbGroup::Kurusuru => write!(f, "Kuru or Suru"),
        }
    }
}

fn get_sticky_stem(verb: &str) -> String {
    "hello".to_string()
}

fn categorise_verb(verb: &str) -> VerbGroup {
    if is_kuru_or_suru(verb) {
        VerbGroup::Kurusuru
    } else if is_ichidan_verb(verb) {
        VerbGroup::Ichidan
    } else {
        VerbGroup::Godan
    }
}

fn categorise_verbs(verbs: Vec<&str>) {
    // This is really for testing
    for i in 0..verbs.len() {
        println!(
            "{} is a {} verb",
            verbs[i],
            categorise_verb(verbs[i]).to_string()
        );
    }
}

fn is_ichidan_verb(verb: &str) -> bool {
    let ichidan_rules: Regex =
        Regex::new(r".*(い|き|び|ち|し|に|み|り|え|け|べ|て|せ|ね|め|れ)る$").unwrap();
    ichidan_rules.is_match(verb)
}

fn is_kuru_or_suru(verb: &str) -> bool {
    let kuru_or_suru: Regex = Regex::new(r"(来|く|す)る$").unwrap();
    kuru_or_suru.is_match(verb)
}

fn is_godan_verb(verb: &str) -> bool {
    !is_ichidan_verb(verb) && !is_kuru_or_suru(verb)
}
