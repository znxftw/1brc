use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
    time::SystemTime,
};
use std::fmt::Display;
use std::io::{BufReader, BufWriter, Lines, Write};

#[derive(Debug)]
struct State {
    max: f64,
    min: f64,
    sum: f64,
    count: u32,
}


impl Default for State {
    fn default() -> Self {
        Self {
            min: f64::MAX,
            max: f64::MIN,
            count: 0,
            sum: 0.0,
        }
    }
}
impl State {
    fn update(&mut self, reading: f64) {
        self.max = self.max.max(reading);
        self.min = self.min.min(reading);
        self.sum += reading;
        self.count += 1;
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let avg = self.sum / (self.count as f64);
        write!(f, "{:.1}/{avg:.1}/{:.1}", self.min, self.max)
    }
}

fn main() {
    let now = SystemTime::now();
    let mut cities = HashMap::<String, State>::new();

    match read_lines("../gen-1brc/measurements.txt") {
        Ok(lines) => {
            process_lines(&mut cities, lines);
        }
        Err(error) => {
            panic!("Error reading file. Error : {}", error);
        }
    }

    let mut file = BufWriter::new(File::create("output.txt").unwrap());

    for (i, (name, state)) in cities.into_iter().enumerate() {
        if i == 0 {
            file.write(format!("{name}={state}").as_bytes()).expect("Unable to write file");
        } else {
            file.write(format!(", {name}={state}").as_bytes()).expect("Unable to write file");
        }
    }
    println!("Finished in {} ms", now.elapsed().unwrap().as_millis());
}

fn process_lines(cities: &mut HashMap<String, State>, lines: Lines<BufReader<File>>) {
    for line in lines.flatten() {
        let vals: Vec<&str> = line.split(';').collect();
        let city = vals[0].to_string();
        let reading: f64 = fast_float::parse(vals[1]).unwrap();
        match cities.get_mut(&city) {
            Some(current_agg) => current_agg.update(reading),
            None => {
                let mut new_agg = State::default();
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

