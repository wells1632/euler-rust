/*
Using names.txt (right click and 'Save Link/Target As...'), a 46K text file containing over
five-thousand first names, begin by sorting it into alphabetical order. Then working out the
alphabetical value for each name, multiply this value by its alphabetical position in the
list to obtain a name score.

For example, when the list is sorted into alphabetical order, COLIN, which is worth
3 + 15 + 12 + 9 + 14 = 53, is the 938th name in the list. So, COLIN would obtain a score of
938 x 53 = 49714.

What is the total of all the name scores in the file?
 */

use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let mut f = File::open("0022_names.txt")?;
    let mut buffer = Vec::new();
    // Print out the description of the problem...
    println!("Using 0022_names.txt, 46K text file containing over");
    println!("five-thousand first names, begin by sorting it into alphabetical order. Then working out the");
    println!("alphabetical value for each name, multiply this value by its alphabetical position in the");
    println!("list to obtain a name score.");
    println!("");
    println!("For example, when the list is sorted into alphabetical order, COLIN, which is worth");
    println!("3 + 15 + 12 + 9 + 14 = 53, is the 938th name in the list. So, COLIN would obtain a score of");
    println!("938 x 53 = 49714.");
    println!("");
    println!("What is the total of all the name scores in the file?");
    
    
    // Read in the file
    f.read_to_end(&mut buffer)?;

    let mut temp_string = String::from_utf8(buffer).expect("Found invalid UTF-8");
    temp_string = temp_string.replace("\"", "");

    let parts = temp_string.split(",");
    let mut collection = parts.collect::<Vec<&str>>();
    let mut max = 0;
    collection.sort();
    for (iter, part) in collection.iter().enumerate() {
        let mut count = 0;
        for a in part.chars() {
            count += a as u32 - 64;
        }
        max += count * (iter as u32 + 1);
    }
    
    println!("Max points: {}", max);

    Ok(())
}
