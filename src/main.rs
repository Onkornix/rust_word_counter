use std::collections::HashMap;
//use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;



// #[derive(Parser)]
// struct Args {
//     /// The pattern to look for
//     input: std::path::PathBuf,
//     /// The path to the file to read
//     output: std::path::PathBuf,
// }

struct Words {
    unordered_map: HashMap<String, u32>,
    ordered_map: HashMap<u32, Vec<String>>
    
}
impl Words {
    fn update_unordered_map(&mut self, line: String) {
        let mut words = line.split_whitespace();
        
        loop {
        
            let next: String = match words.next() {
                Some(s) => s.to_string(), 
                None => break 
            };
            //let m = &mut self.map;

            if self.unordered_map.contains_key(&next) {
                let new_value = match self.unordered_map.remove(&next) {
                    Some(v) => v + 1,
                    None => 0
                };
                self.unordered_map.insert(next, new_value);

            } else {
                self.unordered_map.insert(next, 1);
            }
            
        }
    }
    fn update_ordered_map(&mut self) {
        let value_that_exist: Vec<u32> = {
            let mut values_that_exist: Vec<u32> = Vec::new();
            for value in self.unordered_map.values() {
                if !values_that_exist.contains(value) {
                    values_that_exist.push(*value)
                }
            }
            values_that_exist
        };
        let minimum_occurrence_value = match self.unordered_map.values().min() {
            Some(v) => v,
            None => &0
        };
        let mut current_occurrence_value = match self.unordered_map.values().max() {
            Some(v) => v - 1,
            None => 0
        };
        let mut index: usize = value_that_exist.len();

        println!("{:?}, {}, {}, {}", value_that_exist, minimum_occurrence_value, current_occurrence_value, index);

        while current_occurrence_value >= *minimum_occurrence_value {
            let mut index_list: Vec<String> = Vec::new();

            for word in self.unordered_map.keys() {
                if value_that_exist.contains(&current_occurrence_value) 
                && self.unordered_map.get(word).unwrap() == &current_occurrence_value {
                    index_list.push(word.to_string());
                }

                if !index_list.is_empty() {
                    if self.ordered_map.contains_key(&current_occurrence_value) {
                        let mut old_value = self.ordered_map.get(&current_occurrence_value).unwrap().clone();

                        old_value.append(&mut index_list);

                        self.ordered_map.insert(current_occurrence_value, old_value);
                    }
                    
                }
            } 
            if index == 0 {
                break
            }

            index -= 1;
            current_occurrence_value = *value_that_exist.get(index).unwrap();
             
        }

    
        //self.unordered_map.clear()
    }
}

fn main() {
    //let _args = Args::parse();

    let mut new_map = Words {
        unordered_map: HashMap::new(),
        ordered_map: HashMap::new()
    };

    if let Ok(lines) = read_line("/home/dylan/Documents/text.txt") {
        for line in lines.flatten() {
            new_map.update_unordered_map(line);
            new_map.update_ordered_map();
        }
    }
    // for (word, occurrence) in new_map.unordered_map.iter() {
    //     println!("{}: {}", word, occurrence)
    // }
    
}

fn read_line<P>(filename: P ) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}