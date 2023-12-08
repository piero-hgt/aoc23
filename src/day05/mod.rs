use std::{collections::HashMap, panic::Location};
use text_io::scan;
use std::fs;

#[derive(Debug)]
enum AlmanacStep {
    Seeds,
    SeedsToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

#[derive(Debug, PartialEq)]
struct Range {
    min: u64,
    max: u64,
    add: i64,
}

impl Range {
    fn new(line: &str) -> Self {
        let line = line.trim();

        let mut d_start: String = String::new();
        let mut s_start: String = String::new();
        let mut length: String = String::new();

        scan!(line.bytes() => "{} {} {}\n", d_start, s_start, length);

        let d_start = d_start.parse::<u64>().unwrap();
        let s_start = s_start.parse::<u64>().unwrap();
        let length = length.parse::<u64>().unwrap();

        println!("Range: d_start {}, s_start {}, length {}", d_start, s_start, length);
        let min = s_start;
        let max = s_start + length - 1;
        let add = d_start as i64 - s_start as i64;

        println!("Range for \"{}\": min {}, max {}, add {}", line, min, max, add);

        Range { min, max, add }
    }
}

#[derive(Debug)]
struct RangeSet {
    ranges: Vec<Range>,
}

impl RangeSet {
    fn new() -> Self {
        RangeSet { ranges: Vec::new() }
    }

    fn find_destination(&self, value: u64) -> u64 {
        for range in &self.ranges {
            if value >= range.min && value <= range.max {
                return (value as i64 + range.add) as u64;
            }
        }

        value
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    seeds_to_soil: RangeSet,
    soil_to_fertilizer: RangeSet,
    fertilizer_to_water: RangeSet,
    water_to_light: RangeSet,
    light_to_temperature: RangeSet,
    temperature_to_humidity: RangeSet,
    humidity_to_location: RangeSet,
}

impl Almanac {
    fn new(input: &str) -> Self {
        let mut step = AlmanacStep::Seeds;
        let mut seeds: Vec<u64> = Vec::new();
        let mut seeds_to_soil: RangeSet = RangeSet::new();
        let mut soil_to_fertilizer: RangeSet = RangeSet::new();
        let mut fertilizer_to_water: RangeSet = RangeSet::new();
        let mut water_to_light: RangeSet = RangeSet::new();
        let mut light_to_temperature: RangeSet = RangeSet::new();
        let mut temperature_to_humidity: RangeSet = RangeSet::new();
        let mut humidity_to_location: RangeSet = RangeSet::new();

        for (i, line) in input.lines().enumerate() {
            if line.trim().is_empty() {
                continue;
            }

            if i == 0 {
                seeds = line.replace("seeds:", "").trim().split_whitespace().map(|n| n.parse::<u64>().unwrap()).collect();
                continue;
            }

            match line.trim() {
                "seed-to-soil map:" => {
                    step = AlmanacStep::SeedsToSoil;
                    continue;
                },
                "soil-to-fertilizer map:" => {
                    step = AlmanacStep::SoilToFertilizer;
                    continue;
                },
                "fertilizer-to-water map:" => {
                    step = AlmanacStep::FertilizerToWater;
                    continue;
                },
                "water-to-light map:" => {
                    step = AlmanacStep::WaterToLight;
                    continue;
                },
                "light-to-temperature map:" => {
                    step = AlmanacStep::LightToTemperature;
                    continue;
                },
                "temperature-to-humidity map:" => {
                    step = AlmanacStep::TemperatureToHumidity;
                    continue;
                },
                "humidity-to-location map:" => {
                    step = AlmanacStep::HumidityToLocation;
                    continue;
                },
                _ => {
                    // println!("Step {:?}", step);
                    let range = Range::new(&line);

                    match step {
                        AlmanacStep::Seeds => { }
                        AlmanacStep::SeedsToSoil => { seeds_to_soil.ranges.push(range); }
                        AlmanacStep::SoilToFertilizer => { soil_to_fertilizer.ranges.push(range); }
                        AlmanacStep::FertilizerToWater => { fertilizer_to_water.ranges.push(range); }
                        AlmanacStep::WaterToLight => { water_to_light.ranges.push(range); }
                        AlmanacStep::LightToTemperature => { light_to_temperature.ranges.push(range); }
                        AlmanacStep::TemperatureToHumidity => { temperature_to_humidity.ranges.push(range); }
                        AlmanacStep::HumidityToLocation => { humidity_to_location.ranges.push(range); }
                    }
                }
            }
        }

        Almanac { seeds, seeds_to_soil, soil_to_fertilizer, fertilizer_to_water, water_to_light, light_to_temperature, temperature_to_humidity, humidity_to_location }
    }

    fn find_location_for_seed(&self, seed: u64) -> u64 {
        let soil = self.seeds_to_soil.find_destination(seed);
        let fertilizer = self.soil_to_fertilizer.find_destination(soil);
        let water = self.fertilizer_to_water.find_destination(fertilizer);
        let light = self.water_to_light.find_destination(water);
        let temperature = self.light_to_temperature.find_destination(light);
        let humidity = self.temperature_to_humidity.find_destination(temperature);
        let location = self.humidity_to_location.find_destination(humidity);

        // println!("Seed {} -> Soil {} -> Fertilizer {} -> Water {} -> Light {} -> Temperature {} -> Humidity {} -> Location {}", seed, soil, fertilizer, water, light, temperature, humidity, location);

        location
    }
}

fn generate_ranges(line: &str) -> Range {
    Range::new(line)
}

fn lowest_location_number(input: &str) -> u64 {
    let almanac = Almanac::new(input);
    let mut locations: Vec<u64> = almanac.seeds.iter().map(|seed| almanac.find_location_for_seed(*seed)).collect();
    locations.sort();
    locations[0]
}


pub fn solve_task1() {
    let contents = match fs::read_to_string("input/day05.txt") {
        Ok(contents) => contents,
        Err(e) => panic!("{}", e),
    };

    println!("{}", lowest_location_number(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_ranges() {
        let input = "50 98 2";
        let expected = Range { min: 98, max: 99, add: -48 };
        assert_eq!(generate_ranges(input), expected);

        let input: &str = "39 0 15";
        let expected = Range { min: 0, max: 14, add: 39 };
        assert_eq!(generate_ranges(input), expected);
    }

    #[test]
    fn test_lowest_location_number() {
        let input = "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4";

        assert_eq!(lowest_location_number(input), 35);
    }
}
