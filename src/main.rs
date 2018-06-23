use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut filename = "in.txt";

    if args.len() > 2 {
        panic!("Please enter ONE filename as a param");
    }

    if args.len() == 2 {
        filename = &args[1];
    }

    let file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("Unable to read file {:?}", filename),
    };

    let mut buffer = BufReader::new(file);
    let mut first = String::new();
    buffer
        .read_line(&mut first)
        .expect("Error reading first line");

    let mut countlines = 0;

    for line in buffer.lines() {
        countlines += 1;
        let roda_rebentada = match line {
            Ok(line) => line,
            Err(_) => panic!("Error reading line {}", countlines),
        };
        println!("{}", turn(&roda_rebentada));
    }
}

fn turn(roda: &String) -> String {
    let split = roda.split_whitespace();
    let mut numeros: Vec<i32> = vec![];
    for s in split {
        numeros.push(s.parse::<i32>().unwrap());
    }

    let right: i32 = match numeros[0] > numeros[1] {
        true => 360 - numeros[0] + numeros[1],
        false => (numeros[1] - numeros[0]) % 360,
    };
    let left: i32 = (numeros[0] + (360 - numeros[1])) % 360;

    // println!("{} {} --> {} vs {}", numeros[0], numeros[1], right, left);

    if left > right {
        return "ASCENDENTE".to_string();
    } else if right > left {
        return String::from("DESCENDENTE");
    }

    "DA IGUAL".to_string()
}
