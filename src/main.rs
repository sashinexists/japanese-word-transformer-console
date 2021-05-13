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

#[derive(Serialize, Deserialize)]
enum VerbGroup {
    Ichidan,
    Godan,
    Kurusuru,
}

fn main() {
    //let example_verbs = vec!["買う", "聞く","話す","持つ","死ぬ","飛ぶ","飲む","取る"];
    //categorise_verbs(example_verbs);

    const JOSHI_DATA_FILE_PATH: &str = "src/joshi.json";

    let joshi_data =
        fs::read_to_string(JOSHI_DATA_FILE_PATH).expect("Something went wrong reading the file");

    let joshi_list: Vec<Joshi> = serde_json::from_str(joshi_data.as_str()).unwrap();

    verb_transformation_loop(&joshi_list);
    //println!("{:#?}", joshi_list[3]);

    //verb_categoriser_loop();
}

fn verb_transformation_loop(joshi_list:&Vec<Joshi>) {
    let mut input = String::new();
    while input != "出" {
        input.clear();
        println!("\n動詞を入力下さい　(Please enter a verb) | Alternatively, type 出 to exit\n");
        io::stdin().read_line(&mut input).unwrap();
        input.pop();
        let verb_type = categorise_verb(&input);
        if input != "出" {
            println!("\n何をしたいですか？ (What would you like to do?)\n");
            println!("{}", build_transformation_menu(&joshi_list));
            let mut action = String::new();
            io::stdin().read_line(&mut action).unwrap();
            action.pop();
            let action = action.parse::<usize>().unwrap();
            println!("The {}-form of {} is {}", joshi_list[action].name, input, transform_verb(&input, &joshi_list[action]) );
        }
    }
    println!("さようなら");
}

fn build_transformation_menu(joshi_list:&Vec<Joshi>) -> String {
    let mut output = String::new();
    for i in 0..joshi_list.len() {
        output += &format!("{})  {}  | {}\n", i, joshi_list[i].function, joshi_list[i].name);
    }
    output
}

fn transform_verb(verb: &str, joshi:&Joshi) -> String {
    let verb_group = categorise_verb(verb);
    match verb_group {
        VerbGroup::Ichidan => transform_ichidan_verb(verb, joshi),
        VerbGroup::Godan => transform_godan_verb(verb, joshi),
        VerbGroup::Kurusuru => transform_kuru_suru(verb, joshi)
    }
}

fn transform_ichidan_verb(verb: &str, joshi:&Joshi) -> String{
    let mut sticky_stem = verb.to_string();
    sticky_stem.pop();
    if joshi.ichidan_form=="N/A" {
        sticky_stem + joshi.name.as_str()
    } else {
        sticky_stem + joshi.ichidan_form.as_str()
    }
}

fn transform_godan_verb(verb: &str, joshi:&Joshi) -> String {
    "ITS A GODDAMN GODAN VERB".to_string()
}

fn get_godan_stem(verb: &str, stem:GodanStem) -> String {
    "hello".to_string()
}



fn transform_kuru_suru(verb: &str, joshi:&Joshi) -> String {
    "THIS IS KURU OR SURU".to_string()
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



impl fmt::Display for VerbGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VerbGroup::Ichidan => write!(f, "Ichidan"),
            VerbGroup::Godan => write!(f, "Godan"),
            VerbGroup::Kurusuru => write!(f, "Kuru or Suru"),
        }
    }
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
