use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("calories.txt");
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldnt open {}: {}",display,why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why)=> panic!("could read {}: {}",display,why),
        Ok(_)=> print!("contains:data\n" ),
    }

    let mut split_sting = s.split("\n\n");

    let elf_inventories = split_sting.collect::<Vec<&str>>();
    
    for s in elf_inventories
    {
        let new_string = s.split("\n");
        for i in new_string{
            print!("{}\t",i);
        }
        print!("\n");
    }
}