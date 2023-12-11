use text_io::scan;
use std::fs;
use std::ops::Range;
use rayon::prelude::*;

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
struct MapRange {
    min: u64,
    max: u64,
    add: i64,
}

impl MapRange {
    fn new(line: &str) -> Self {
        let line = line.trim();

        let mut d_start: String = String::new();
        let mut s_start: String = String::new();
        let mut length: String = String::new();

        scan!(line.bytes() => "{} {} {}\n", d_start, s_start, length);

        let d_start = d_start.parse::<u64>().unwrap();
        let s_start = s_start.parse::<u64>().unwrap();
        let length = length.parse::<u64>().unwrap();

        let min = s_start;
        let max = s_start + length - 1;
        let add = d_start as i64 - s_start as i64;

        MapRange { min, max, add }
    }
}

#[derive(Debug)]
struct SeedRange {
    min: u64,
    max: u64,
}

impl SeedRange {
    fn get_range(&self) -> Range<u64> {
        self.min..self.max
    }
}

#[derive(Debug)]
struct MapRangeSet {
    ranges: Vec<MapRange>,
}

impl MapRangeSet {
    fn new() -> Self {
        MapRangeSet { ranges: Vec::new() }
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
    seeds: Vec<SeedRange>,
    seeds_to_soil: MapRangeSet,
    soil_to_fertilizer: MapRangeSet,
    fertilizer_to_water: MapRangeSet,
    water_to_light: MapRangeSet,
    light_to_temperature: MapRangeSet,
    temperature_to_humidity: MapRangeSet,
    humidity_to_location: MapRangeSet,
}

impl Almanac {
    fn new(input: &str) -> Self {
        let mut step = AlmanacStep::Seeds;
        let mut seeds: Vec<SeedRange> = Vec::new();
        let mut seeds_to_soil: MapRangeSet = MapRangeSet::new();
        let mut soil_to_fertilizer: MapRangeSet = MapRangeSet::new();
        let mut fertilizer_to_water: MapRangeSet = MapRangeSet::new();
        let mut water_to_light: MapRangeSet = MapRangeSet::new();
        let mut light_to_temperature: MapRangeSet = MapRangeSet::new();
        let mut temperature_to_humidity: MapRangeSet = MapRangeSet::new();
        let mut humidity_to_location: MapRangeSet = MapRangeSet::new();

        for (i, line) in input.lines().enumerate() {
            if line.trim().is_empty() {
                continue;
            }

            if i == 0 {
                // seeds = line.replace("seeds:", "").trim().split_whitespace().map(|n| n.parse::<u64>().unwrap()).collect();
                let values: Vec<u64> = line.replace("seeds:", "").trim().split_whitespace().map(|n| n.parse::<u64>().unwrap()).collect();

                let mut i: u64 = 0;

                while i < values.len() as u64 {
                    let min = values[i as usize];
                    let max = min + values[(i + 1) as usize] - 1;
                    seeds.push(SeedRange { min, max });
                    i += 2;
                }

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
                    let range = MapRange::new(&line);

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

    fn find_smallest_location(&self) -> u64 {
        let mut location = u64::MAX;

        for seed_range in &self.seeds {
            println!("Seed range {:?}", seed_range);

            let mut locations: Vec<u64> = seed_range
                .get_range()
                .into_par_iter()
                .map(|seed| self.find_location_for_seed(seed))
                .collect::<Vec<u64>>();
            locations.sort();
            if locations[0] < location {
                println!("New location {}", locations[0]);
                location = locations[0];
            }
        }
        location
    }

    fn get_seeds(&self) -> Vec<u64> {
        let mut seeds: Vec<u64> = Vec::new();

        for seed_range in &self.seeds {
            for seed in seed_range.get_range() {
                seeds.push(seed);
            }
        }

        seeds
    }
}

fn generate_ranges(line: &str) -> MapRange {
    MapRange::new(line)
}

fn lowest_location_number(input: &str) -> u64 {
    let almanac = Almanac::new(input);
    almanac.find_smallest_location()
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
        let expected = MapRange { min: 98, max: 99, add: -48 };
        assert_eq!(generate_ranges(input), expected);

        let input: &str = "39 0 15";
        let expected = MapRange { min: 0, max: 14, add: 39 };
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

        assert_eq!(lowest_location_number(input), 46);
    }
}
