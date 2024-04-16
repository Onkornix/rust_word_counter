use std::collections::HashMap;
//use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
pub mod words;



// #[derive(Parser)]
// struct Args {
//     /// The pattern to look for
//     input: std::path::PathBuf,
//     /// The path to the file to read
//     output: std::path::PathBuf,
// }

fn main() {
    //let _args = Args::parse();

    let mut new_map = words::Words {
        ungrouped_map: HashMap::new(),
        grouped_map: HashMap::new(),
        values_that_exist: Vec::new(),
    };

    if let Ok(lines) = read_line("/home/dylan/Documents/paradise.txt") {
        for line in lines.flatten() {
            new_map.update_ungrouped_map(line);
            
        }
    } 
    new_map.create_grouped_map();
    new_map.write_to_file(&String::from("/home/dylan/code/rust/worderer/output.txt"));
    //print!("{:?}", new_map.grouped_map);
    // for (word, occurrence) in new_map.unordered_map.iter() {
    //     println!("{}: {}", word, occurrence)
    // }
    
}

fn read_line<P>(filename: P ) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}