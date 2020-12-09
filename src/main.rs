use std::{collections::{HashMap, HashSet}, fmt::Debug, fs::File};
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
    contains: Vec<(i32, usize)>, // number, index_in_array
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

    fn get_bag_kind(&self, color: &str) -> &KindOfBag {
        let index_in_vec = self.index_by_color.get(color).unwrap();
        self.kind_of_bags_by_index.get(*index_in_vec).unwrap()
    }

    fn add_rule(&mut self, container_color: &str, count: i32, containee_color: &str) {
        let container = self.get_or_create_bag_kind(container_color.to_string());
        let containee = self.get_or_create_bag_kind(containee_color.to_string());
        let a = self.kind_of_bags_by_index.get_mut(container).unwrap();
        a.add_containee(count, containee);
    }

    fn can_contain(&self, container_bag: &KindOfBag, bag: &KindOfBag) -> bool {
        let index_bag = self.index_by_color.get(&bag.color).unwrap();
        let res = container_bag.contains.iter().find(|(_, bag_id)| *bag_id == *index_bag);
        //println!("can_contain search for id {}\n\t{:?}\n\t{:?}\n\t{:?}", index_bag, container_bag, bag, res);
        res.is_some()
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

        let searched = rules.get_bag_kind("shiny gold");
        let mut found: HashSet<String> = HashSet::new();
        found.insert(searched.color.to_string());
        let mut previous_found_num = 0;
        let mut found_num = found.len();

        while found_num != previous_found_num {
            let mut found2: HashSet<String> = HashSet::new();
            for bag in &rules.kind_of_bags_by_index {
                for f in &found {
                    //println!("can {} contain {}?", bag.color, f);
                    let f = rules.get_bag_kind(f);
                    if rules.can_contain(&bag, f) {
                        found2.insert(bag.color.clone());
                        //println!("\tyes!");
                    }
                }
            }
            println!("found this round: {:?}", found2);
            found = found.union(&found2).cloned().collect();
            println!("found: {:?}", found);
            previous_found_num = found_num;
            found_num = found.len();
        }

        // len()-1 because we remove the actual searched bag from the set
        println!("{} way to get a {}", found.len() - 1, searched.color);
    }
}
