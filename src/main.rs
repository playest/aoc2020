use std::{collections::HashMap, fmt::Debug, fs::File};
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct KindOfBag {
    color: String,
    contains: Vec<(i32, usize)>,
}

impl<'a> KindOfBag {
    fn new(s: String) -> Self {
        let contains: Vec<(i32, usize)> = Vec::new();
        //contains.push((count, containee_id));
        KindOfBag {
            color: s,
            contains,
        }
    }

    fn add_containee(&mut self, count: i32, index: usize) {
        self.contains.push((count, index));
    }
}

struct BagRules {
    kind_of_bags_by_index: Vec<KindOfBag>,
    index_by_color: HashMap<String, usize>,
}

impl BagRules {
    fn new() -> Self {
        BagRules {
            kind_of_bags_by_index: Vec::new(),
            index_by_color: HashMap::new(),
        }
    }

    fn get_or_create_bag_kind(&mut self, color: String) -> usize {
        let index_in_vec = self.index_by_color.get(&color);
        match index_in_vec {
            None => {
                let bag_kind = KindOfBag::new(color.to_string());
                self.kind_of_bags_by_index.push(bag_kind);
                let i = self.kind_of_bags_by_index.len() - 1;
                self.index_by_color.insert(color.to_string(), i);
                i
            },
            Some(index) => *index
        }
    }

    fn add_rule(&mut self, container_color: &str, count: i32, containee_color: &str) {
        let container = self.get_or_create_bag_kind(container_color.to_string());
        let containee = self.get_or_create_bag_kind(containee_color.to_string());
        let a = self.kind_of_bags_by_index.get_mut(container).unwrap();
        a.add_containee(count, containee);
    }
}

impl Debug for BagRules {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, kind_of_bag) in self.kind_of_bags_by_index.iter().enumerate() {
            writeln!(f, "{}: {:?}", i, kind_of_bag.color)?;
            for (count, index) in &kind_of_bag.contains {
                let containee = self.kind_of_bags_by_index.get(*index).unwrap();
                writeln!(f, "\t{} x {}", count, containee.color)?;
            }
        }
        Ok(())
    }
}

fn main() {
    if let Ok(lines) = read_lines("./inputs/input_day7.txt") {
        let mut rules = BagRules::new();
        for line in lines {
            if let Ok(line) = line {
                let words: Vec<&str> = line.split(" contain ").collect();
                //println!("{:?}", words);
                
                let color: Vec<&str> = words[0].split_whitespace().collect();
                let color = &color[0..2].join(" ");
                println!("{}", color);
                
                let contains: Vec<&str> = words[1].trim_end_matches('.').split(", ").collect();
                //println!("\tcolor {:?}", contains);
                for content in contains {
                    if content == "no other bags" {
                        println!("\t.");
                    }
                    else {
                        let content: Vec<&str> = content.split_whitespace().collect();
                        let num = content[0].parse::<i32>().unwrap();
                        let color2 = content[1..3].join(" ");
                        println!("\t{} {}", num, color2);
                        rules.add_rule(color, num, &color2)
                    }
                }
            }
        }
        println!("Rules:\n{:?}", rules);
    }
}
