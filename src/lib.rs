use std::{
    collections::HashMap, fmt, fs::File, io::{BufRead, BufReader}
};

//  Sarajevo;4.5
//  Brazzaville;12.6
//  Almaty;2.7
//  Beijing;32.7
//  Oranjestad;22.0
//  Saint-Pierre;0.5
//  Lahore;32.0
//  Dodoma;13.2
//  Gabès;25.9
//  Anchorage;0.6

// {Adelaide=15.0/15.0/15.0, Cabo San Lucas=14.9/14.9/14.9, Dodoma=22.2/22.2/22.2, Halifax=12.9/12.9/12.9, Karachi=15.4/15.4/15.4, Pittsburgh=9.7/9.7/9.7, Ségou=25.7/25.7/25.7, Tauranga=38.2/38.2/38.2, Xi'an=24.2/24.2/24.2, Zagreb=12.2/12.2/12.2}

// measurements-3.txt
// Bosaso;5.0
// Bosaso;20.0
// Bosaso;-5.0
// Bosaso;-15.0
// Petropavlovsk-Kamchatsky;9.5
// Petropavlovsk-Kamchatsky;-9.5
// OUT:
// {Bosaso=-15.0/1.3/20.0, Petropavlovsk-Kamchatsky=-9.5/0.0/9.5}

// github.com/gunnarmorling/1brc
//   https://github.com/gunnarmorling/1brc/tree/main/src/test/resources/samples
// https://andrewodendaal.com/rust-file-io/

// The text file contains temperature values for a range of weather stations. Each row is one measurement in the format <string: station name>;<double: measurement>, with the measurement value having exactly one fractional digit. The following shows ten rows as an example:
//
//
//The task is to write a Java program which reads the file, calculates the min, mean, and max temperature value per weather station, and emits the results on stdout like this (i.e. sorted alphabetically by station name, and the result values per station in the format <min>/<mean>/<max>, rounded to one fractional digit):
//
// Hamburg;12.0
// Bulawayo;8.9
// Palembang;38.8
// St. John's;15.2
// Cracow;12.6
// Bridgetown;26.9
// Istanbul;6.2
// Roseau;34.4
// Conakry;31.2
// Istanbul;23.0
//
//{Abha=-23.0/18.0/59.2, Abidjan=-16.2/26.0/67.3, Abéché=-10.0/29.4/69.0, Accra=-10.1/26.4/66.4, Addis Ababa=-23.7/16.0/67.0, Adelaide=-27.8/17.3/58.5, ...}
//

// rg Budapest measurements.txt | wc -l
// 2422167

type StationName = String;

#[derive(Debug)]
struct Temperature {
    min: f32,
    max: f32,
    mean: f32,
    n: f32,
}

impl Temperature {
    fn new(temperature: f32) -> Self {
        Self {
            min: temperature,
            max: temperature,
            mean: temperature,
            n: 0.,
        }
    }
}

// RES: Budapest=-36.7/11.3/61.2, Yinchuan=-37.3/9.0/59.3, Zagreb=-39.5/10.7/65.6
// My:
// Some(Temperature { min: -36.7, max: 61.2, mean: 11.309869, n: 2422166.0 })
// Some(Temperature { min: -39.5, max: 65.6, mean: 10.706982, n: 2420671.0 })
// Some(Temperature { min: -37.3, max: 59.3, mean: 9.006653, n: 2422896.0 })
// Done in: `70s`


pub fn read_and_calculated_measuerements() -> Result<(), Box<dyn std::error::Error>> {
    let measurements = File::open("measurements.txt")?;

    let reader = BufReader::with_capacity(32768, measurements);

    let mut outs: HashMap<StationName, Temperature> = HashMap::new();

    // Read line by line
    for line in reader.lines() {
        let line = line?;

        if let Some((name, temperature)) = line.split_once(';') {
            let temperature: f32 = temperature.parse()?;
            if let Some(station_temperature) = outs.get_mut(name) {
                station_temperature.n += 1.;
                station_temperature.mean += (temperature - station_temperature.mean) / station_temperature.n;
                station_temperature.max = station_temperature.max.max(temperature);
                station_temperature.min = station_temperature.max.min(temperature);
            } else {
                outs.insert(name.to_string(), Temperature::new(temperature));
            }
        }

        // println!("{}", line);
    }
    println!("{:?}", outs.get("Budapest"));
    println!("{:?}", outs.get("Yinchuan"));
    println!("{:?}", outs.get("Zagreb"));

    // println!("Hello, world!: {:?}", measurements);

    Ok(())
}
