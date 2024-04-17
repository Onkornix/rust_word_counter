use std::collections::HashMap;
//use clap::Parser;
use std::fs::File;
use std::io::{BufWriter, Write};

pub struct Words {
    pub ungrouped_map: HashMap<String, u32>,
    pub grouped_map: HashMap<u32, Vec<String>>,
    pub values_that_exist: Vec<u32>,
    
}
impl Words {
    pub fn update_ungrouped_map(&mut self, line: String) {
        let mut words = line.split_whitespace();
        
        loop {
        
            let next: String = match words.next() {
                Some(s) => s.to_string(), 
                None => break 
            };
            
            

            if self.ungrouped_map.contains_key(&next) {
                let new_value = match self.ungrouped_map.remove(&next) {
                    Some(v) => v + 1,
                    None => 0
                };
                self.ungrouped_map.insert(next, new_value);

            } else {
                self.ungrouped_map.insert(next, 1);
            }
            
        }
    }
    pub fn create_grouped_map(&mut self) {
        self.values_that_exist = {
            let mut values_that_exist: Vec<u32> = Vec::new();
            for value in self.ungrouped_map.values() {
                if !values_that_exist.contains(value) {
                    values_that_exist.push(*value)
                }
            }
            values_that_exist
        };
        let minimum_occurrence_value = match self.ungrouped_map.values().min() {
            Some(v) => v,
            None => &0
        };
        let mut current_occurrence_value = match self.ungrouped_map.values().max() {
            Some(v) => v - 1,
            None => 0
        };
        let mut index: usize = self.values_that_exist.len();

        

        while current_occurrence_value >= *minimum_occurrence_value {
            let mut index_list: Vec<String> = Vec::new();

            for word in self.ungrouped_map.keys() {
                if self.ungrouped_map.get(word).unwrap() == &current_occurrence_value {
                    index_list.push(word.to_string());
                }

                
            } 
            if !index_list.is_empty() {
                if self.grouped_map.contains_key(&current_occurrence_value) {

                    for word in self.grouped_map.get(&current_occurrence_value).unwrap() {
                        if index_list.contains(&word) {
                            continue
                        } else {
                            index_list.push(word.to_string())
                        }
                    }
                } else {
                    self.grouped_map.insert(current_occurrence_value, index_list);
                }
                
            }

            
            if index == 0 {
                break
            }

            index -= 1;
            current_occurrence_value = *self.values_that_exist.get(index).unwrap();
        }

    
        self.ungrouped_map.clear()
    }
    pub fn write_to_file(&mut self, output: &String) {
        
        let mut writer = BufWriter::new(File::create(output).expect("epic fail lol"));


        let mut all_vals_sorted = self.values_that_exist.clone();
        all_vals_sorted.sort();
        all_vals_sorted.reverse();
        let mut all_vals_sorted = all_vals_sorted.iter();
        let mut current_value = all_vals_sorted.next().expect("msg");

        //println!("{current_value}, {max_value} {:?}", all_vals_sorted);
        loop {
            for occ_value in self.grouped_map.keys() {
                if occ_value == current_value {
                    let data = format!("{}:\n{}", occ_value, self.grouped_map.get(occ_value).expect("fail").join("\n"));
                    writer.write(data.as_bytes()).expect("failed to write");
                    writer.write("\n\n".as_bytes()).expect("failed to write");
                    break
                }
                
                
            }
            current_value = match all_vals_sorted.next() {
                Some(v) => v,
                None => break
            };
        }
        
    }
}
