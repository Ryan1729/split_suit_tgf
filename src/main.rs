use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let reader = BufReader::new(File::open("splitsuitcards.csv").expect(
        "Cannot open splitsuitcards.csv",
    ));

    let path = Path::new("splitsuit.tgf");
    let display = path.display();

    let mut output = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut triples = Vec::new();
    for l in reader.lines() {
        let line = l.unwrap();
        let mut chunks = line.split(",");

        triples.push((
            chunks.next().unwrap().to_owned(),
            chunks.next().unwrap().to_owned(),
            chunks.next().unwrap().to_owned(),
        ));
    }

    let mut result = String::new();


    for (index, &(ref value, ref suit1, ref suit2)) in triples.iter().enumerate() {
        result.push_str(&format!("{} {}:{}:{}\n", index, value, suit1, suit2));
    }

    result.push_str("#\n");

    for (index1, ref t1) in triples.iter().enumerate() {
        for (index2, ref t2) in triples.iter().enumerate() {
            if index1 != index2 {
                if t1.1 == t2.1 || t1.2 == t2.2 {
                    result.push_str(&format!("{} {}\n", index1, index2));
                }
            }
        }
    }

    match output.write_all(result.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
        Ok(_) => println!("successfully wrote to {}", display),
    }

}
