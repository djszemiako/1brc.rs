use std::collections::BTreeMap;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Metrics {
    min: f32,
    max: f32,
    count: i32,
    sum: f32,
}

impl Metrics {
    fn new(measurement: f32) -> Self {
        Metrics {
            min: measurement,
            max: measurement,
            count: 1,
            sum: measurement,
        }
    }
}

impl fmt::Display for Metrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}/{}/{}",
            self.min,
            self.sum / self.count as f32,
            self.max
        )
    }
}

// https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;

    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let mut stations: BTreeMap<String, Metrics> = BTreeMap::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines.map_while(Result::ok) {
            if line.starts_with("#") {
                continue;
            }

            let (station, raw_measurement) = line
                .split_once(";")
                .expect(format!("Could not split line: {line}").as_str());

            let measurement = raw_measurement
                .parse::<f32>()
                .expect(format!("Could not parse measurement: {raw_measurement}").as_str());

            if !stations.contains_key(station) {
                println!("Station '{station}' does not exist, adding...");

                stations.insert(station.to_string(), Metrics::new(measurement));
            } else {
                if let Some(x) = stations.get_mut(station) {
                    if measurement > x.max {
                        x.max = measurement;
                    }

                    if measurement < x.min {
                        x.min = measurement;
                    }

                    x.sum = x.sum + measurement;

                    x.count = x.count + 1;
                }
            }
        }
    }

    println!("Iterating over {} stations...", stations.len());

    for (k, v) in stations.iter() {
        println!("Station: {}, Measumrents: {}", k, v,);
    }
}
