use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;

    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let mut stations: HashMap<String, Vec<f32>> = HashMap::new();

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

                let measurements: Vec<f32> = vec![measurement];

                stations.insert(station.to_string(), measurements);
            } else {
                if let Some(x) = stations.get_mut(station) {
                    x.push(measurement);
                }
            }
        }
    }

    println!("Iterating over {} stations...", stations.len());

    for (k, v) in stations.iter() {
        println!(
            "Station: {}, Measumrents: {}",
            k,
            v.into_iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join(",")
        );
    }
}
