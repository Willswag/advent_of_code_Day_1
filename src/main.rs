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

    let mut elf_number =0;
    let mut the_fat_elf = 0;
    let mut fat_elf_cals = 0;

    for s in elf_inventories
    {
        print!("elf {} has these items:\n",elf_number);
        let mut total_cals = 0;
        let new_string = s.split("\n");
        for i in new_string{
            print!("{}\t",i);
            let item_cals:i32= i.trim().parse().expect("needed a number");
            total_cals = total_cals + item_cals;
        }
        print!("\nelf has {} calories\n",total_cals);
        print!("\n");
        elf_number = elf_number+1;
    }
}