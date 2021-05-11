use regex::Regex;
use std::{fmt, io, str};

fn main() {
    //categorise_verbs(vec!["行く","来る","食べる","歌う","起きる","話す","する","分かる","答える","くる" ])
    let mut input = String::new();
    while input != "閉じる" {
        input.clear();
        println!("動詞を入力下さい　(Please enter a verb) | Alternatively, type 出る to exit");
        io::stdin().read_line(&mut input).unwrap();
        input.pop();
        if input != "出る" {
            categorise_verbs(vec![input.as_str()]);
        }
    }
    println!("さようなら")
}

enum VerbGroup {
    Ichidan,
    Godan,
    Kurusuru,
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
