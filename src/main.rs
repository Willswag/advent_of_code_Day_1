use std::fs::File;
use std::io::prelude::*;
use std::ops::Index;
use std::path::Path;

struct Elf{
    number: i32,
    calories: i32
}

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

    let split_sting = s.split("\n\n");

    let elf_inventories = split_sting.collect::<Vec<&str>>();

    let mut elves:Vec<Elf> = Vec::new();


    let mut elf_cals:Vec<i32> = Vec::new();

    let mut elf_number =1;
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
            elf_cals.push(total_cals);
        }
        elves.push(Elf { number: elf_number, calories: total_cals });
        if total_cals > fat_elf_cals {
            the_fat_elf = elf_number;
            fat_elf_cals = total_cals;
        }
        print!("\nelf has {} calories\n",total_cals);
        print!("\n");
        elf_number = elf_number+1;
    }

    elf_cals.sort();
    let n = elf_cals.len();
    let total_cals = elf_cals.index(elf_cals.len())

    // for cals in elf_cals {
    //     print!("{}\n",cals);
    // }

    // let mut total_cal =0;
    // for i in 0..2 {
    //     let mut big_cal = 0;
    //     let mut big_cal_number = 0; 
    //     for elf in elves
    //     {
    //         if elf.calories > big_cal{
    //             big_cal = elf.calories;
    //             big_cal_number = elf.number;
    //         }
    //         print!("the fat elf is elf number {} and he has {} calories\n",the_fat_elf,fat_elf_cals );

    //     }

    // }


}