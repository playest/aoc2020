use std::{fs::File};
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
enum Height {
    Inches(i32),
    Centimers(i32),
}

#[derive(Debug)]
struct Document {
    birth_year: Option<i32>, // byr
    issue_year: Option<i32>, // iyr
    expiration_year: Option<i32>, // eyr
    height: Option<Height>, // hgt
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
        let birth_year_check = self.birth_year.map_or(false, |by| by >= 1920 && by <= 2002);
        let issue_year_check = self.issue_year.map_or(false, |by| by >= 2010 && by <= 2020);
        let expiration_year_check = self.expiration_year.map_or(false, |by| by >= 2020 && by <= 2030);
        let height_check = self.height.as_ref().map_or(false, |h| match h {
            Height::Centimers(h) => *h >= 150 && *h <= 193,
            Height::Inches(h) => *h >= 59 && *h <= 76,
        });

        let hexa: &[_] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];
        let hair_color_check = self.hair_color.as_ref().map_or(false, |hc| hc.len() == 7 && hc.trim_end_matches(hexa).eq("#"));

        let eye_color_check = self.eye_color.as_ref().map_or(false, |ec| ec.eq("amb") || ec.eq("blu") || ec.eq("brn") || ec.eq("gry") || ec.eq("grn") || ec.eq("hzl") || ec.eq("oth"));

        let numbers: &[_] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        let passport_id_check = self.passport_id.as_ref().map_or(false, |pi| pi.len() == 9 && pi.trim_start_matches(numbers).is_empty());

        let valid = birth_year_check && issue_year_check && expiration_year_check && height_check && hair_color_check && eye_color_check && passport_id_check;

        println!("valid?:\n\tbirth_year: {}\n\tissue_year: {}\n\texpiration_year: {}\n\theight: {}\n\thair_color: {}\n\teye_color: {}\n\tpassport_id: {}\n\t=> valid: {}", birth_year_check, issue_year_check, expiration_year_check, height_check, hair_color_check, eye_color_check, passport_id_check, valid);

        valid
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
                                current_doc.birth_year = Some(value.parse::<i32>().unwrap())
                            },
                            "iyr" => {
                                if current_doc.issue_year.is_some() {
                                    panic!("Do not erase current_doc.issue_year: {:?}", current_doc.issue_year);
                                }
                                current_doc.issue_year = Some(value.parse::<i32>().unwrap())
                            },
                            "eyr" => {
                                if current_doc.expiration_year.is_some() {
                                    panic!("Do not erase current_doc.expiration_year: {:?}", current_doc.expiration_year);
                                }
                                current_doc.expiration_year = Some(value.parse::<i32>().unwrap())
                            },
                            "hgt" => {
                                if current_doc.height.is_some() {
                                    panic!("Do not erase current_doc.height: {:?}", current_doc.height);
                                }
                                if value.ends_with("cm") {
                                    current_doc.height = Some(Height::Centimers(value.trim_end_matches("cm").parse::<i32>().unwrap()))
                                }
                                else if value.ends_with("in") {
                                    current_doc.height = Some(Height::Inches(value.trim_end_matches("in").parse::<i32>().unwrap()))
                                }
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
        for doc in &docs {
            println!("- {:?}", doc);
            if doc.is_valid() {
                valid_count += 1;
            }
        }

        println!("doc_count: {}, valid_count: {}", docs.len(), valid_count);
        
    }
}
