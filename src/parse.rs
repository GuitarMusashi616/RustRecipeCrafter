use std::fs::File;
use csv::Reader;

#[derive(Debug)]
pub struct Pattern {
    pub name: String,
    pub amount: u32,
    pub ingredients: [Option<String>; 9],
}

pub fn string_to_option(option: &str) -> Option<String> {
    match option {
        "" => None,
        _ => Some(option.to_string()),
    }
}

pub fn csv_to_manual_struct() -> Vec<Pattern> {
    let rdr: Reader<File> = Reader::from_path("recipes.csv").expect("file to be there");
    let mut patterns: Vec<Pattern> = vec![];
    for record in rdr.into_records() {
        let record = record.expect("CSV row invalid");

        let name = record[0].to_string();
        let amount = record[1].parse().expect("number expected");

        let ingredients = [
            string_to_option(&record[2]),
            string_to_option(&record[3]),
            string_to_option(&record[4]),
            string_to_option(&record[5]),
            string_to_option(&record[6]),
            string_to_option(&record[7]),
            string_to_option(&record[8]),
            string_to_option(&record[9]),
            string_to_option(&record[10]),
        ];

        let pattern = Pattern { name, amount, ingredients};
        patterns.push(pattern);
    }
    patterns
}