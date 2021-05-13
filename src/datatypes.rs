use std::{collections::BTreeMap, fs, str::{self}};
#[derive(Serialize, Deserialize, Debug)]
pub struct Joshi {
    pub name: String,
    pub function: String,
    pub joins_with: Vec<WordType>,
    pub godan_stem: Vowel,
    pub ichidan_form: String,
    pub word_type: WordType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Kana {
    pub name: String,
    pub consonant: Consonant,
    pub vowel: Vowel,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum WordType {
    Meishi,
    Doushi,
    Keiyoushi,
    Joshi,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Vowel {
    A,
    I,
    U,
    E,
    O,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Consonant {
    None,
    K,
    S,
    T,
    N,
    B,
    M,
    R,
}

#[derive(Serialize, Deserialize)]
pub enum VerbGroup {
    Ichidan,
    Godan,
    Kurusuru,
}

pub fn joshi_list() -> Vec<Joshi>{
    const JOSHI_DATA_FILE_PATH: &str = "src/joshi.json";
    let joshi_data =
        fs::read_to_string(JOSHI_DATA_FILE_PATH).expect("Something went wrong reading the joshi file");

    let joshi_list: Vec<Joshi> = serde_json::from_str(joshi_data.as_str()).unwrap();
    joshi_list
}



pub fn kana_chart() -> BTreeMap<char,(Vowel,Consonant)> {
    const KANA_CHART_FILE_PATH: &str = "src/kana.json";

    let kana_data = fs::read_to_string(KANA_CHART_FILE_PATH)
        .expect("Something went wrong reading the kana file");

    let kana_chart: Vec<Kana> = serde_json::from_str(kana_data.as_str()).unwrap();

    let mut kana_map:BTreeMap<char,(Vowel,Consonant)> = BTreeMap::new();

    for kana in kana_chart {
        let kana_name = kana.name.to_string().pop().unwrap();
        kana_map.insert(kana_name, (kana.vowel,kana.consonant));
    }

    kana_map
}