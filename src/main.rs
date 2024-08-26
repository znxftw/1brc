use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
    time::SystemTime,
};
use std::io::{BufReader, Lines};

use serde::Serialize;

#[derive(Serialize)]
struct ReadingAggregate {
    max: f64,
    min: f64,
    sum: f64,
    count: u32,
}

impl ReadingAggregate {
    fn new() -> ReadingAggregate {
        ReadingAggregate {
            max: f64::MIN,
            min: f64::MAX,
            sum: 0.0,
            count: 0,
        }
    }
    fn update(&mut self, reading: f64) {
        self.max = self.max.max(reading);
        self.min = self.min.min(reading);
        self.sum += reading;
        self.count += 1;
    }
}

fn main() {
    let now = SystemTime::now();
    let mut cities = HashMap::<String, ReadingAggregate>::new();

    match read_lines("../gen-1brc/measurements.txt") {
        Ok(lines) => {
            process_lines(&mut cities, lines);
        }
        Err(error) => {
            panic!("Error reading file. Error : {}", error);
        }
    }

    let mut output_map = HashMap::<String, (f64, f64, f64)>::new();
    for (key, value) in cities.into_iter() {
        let tuple = (value.min, value.sum / value.count as f64, value.max);
        output_map.insert(key, tuple);
    }

    let file = File::create("output.txt").unwrap();
    let _ = serde_json::to_writer(file, &output_map);

    println!("Finished in {} ms", now.elapsed().unwrap().as_millis());
}

fn process_lines(cities: &mut HashMap<String, ReadingAggregate>, lines: Lines<BufReader<File>>) {
    for line in lines.flatten() {
        let vals: Vec<&str> = line.split(';').collect();
        let city = vals[0].to_string();
        let reading: f64 = fast_float::parse(vals[1]).unwrap();
        match cities.get_mut(&city) {
            Some(current_agg) => current_agg.update(reading),
            None => {
                let mut new_agg = ReadingAggregate::new();
                new_agg.update(reading);
                cities.insert(city, new_agg);
            }
        }
    }
}

// Source : rust by example
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

