use std::{fs::File};
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct Document {
    birth_year: Option<String>, // byr
    issue_year: Option<String>, // iyr
    expiration_year: Option<String>, // eyr
    height: Option<String>, // hgt
    hair_color: Option<String>, // hcl
    eye_color: Option<String>, // ecl
    passport_id: Option<String>, // pid
    country_id: Option<String>, // cid
}

impl Document {
    fn new() -> Document {
        Document {
            birth_year: None,
            issue_year: None,
            expiration_year: None,
            height: None,
            hair_color: None,
            eye_color: None,
            passport_id: None,
            country_id: None,
        }
    }

    fn is_valid(&self) -> bool {
        self.birth_year.is_some() &&
        self.issue_year.is_some() &&
        self.expiration_year.is_some() &&
        self.height.is_some() &&
        self.hair_color.is_some() &&
        self.eye_color.is_some() &&
        self.passport_id.is_some() &&
        //self.country_id.is_some() &&
        true
    }
}

fn main() {
    if let Ok(lines) = read_lines("./inputs/input_day4.txt") {
        let mut docs: Vec<Document> = Vec::new();
        let mut current_doc: Document = Document::new();
        for line in lines {
            if let Ok(line) = line {
                if line.is_empty() {
                    docs.push(current_doc);
                    current_doc = Document::new();
                }
                else {
                    for field_and_value in line.split(" ") {
                        println!("{}", field_and_value);
                        let parts: Vec<&str> = field_and_value.split(":").collect();
                        let field_name = parts[0];
                        let value = parts[1];
                        match field_name {
                            "byr" => {
                                if current_doc.birth_year.is_some() {
                                    panic!("Do not erase current_doc.birth_year: {:?}", current_doc.birth_year);
                                }
                                current_doc.birth_year = Some(String::from(value))
                            },
                            "iyr" => {
                                if current_doc.issue_year.is_some() {
                                    panic!("Do not erase current_doc.issue_year: {:?}", current_doc.issue_year);
                                }
                                current_doc.issue_year = Some(String::from(value))
                            },
                            "eyr" => {
                                if current_doc.expiration_year.is_some() {
                                    panic!("Do not erase current_doc.expiration_year: {:?}", current_doc.expiration_year);
                                }
                                current_doc.expiration_year = Some(String::from(value))
                            },
                            "hgt" => {
                                if current_doc.height.is_some() {
                                    panic!("Do not erase current_doc.height: {:?}", current_doc.height);
                                }
                                current_doc.height = Some(String::from(value))
                            },
                            "hcl" => {
                                if current_doc.hair_color.is_some() {
                                    panic!("Do not erase current_doc.hair_color: {:?}", current_doc.hair_color);
                                }
                                current_doc.hair_color = Some(String::from(value))
                            },
                            "ecl" => {
                                if current_doc.eye_color.is_some() {
                                    panic!("Do not erase current_doc.eye_color: {:?}", current_doc.eye_color);
                                }
                                current_doc.eye_color = Some(String::from(value))
                            },
                            "pid" => {
                                if current_doc.passport_id.is_some() {
                                    panic!("Do not erase current_doc.passport_id: {:?}", current_doc.passport_id);
                                }
                                current_doc.passport_id = Some(String::from(value))
                            },
                            "cid" => {
                                if current_doc.country_id.is_some() {
                                    panic!("Do not erase current_doc.country_id: {:?}", current_doc.country_id);
                                }
                                current_doc.country_id = Some(String::from(value))
                            },
                            _ => panic!("Unrecognized field: {}", field_name),
                        }
                    }
                }
            }
        }
        docs.push(current_doc);

        let mut valid_count = 0;
        for doc in docs {
            println!("- {:?}", doc);
            if doc.is_valid() {
                valid_count += 1;
            }
        }

        println!("valid_count: {}", valid_count);
        
    }
}
