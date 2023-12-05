use std::ops::Range;

fn main() {
    let (
        humidity_location,
        temperature_humidity,
        light_temperature,
        water_light,
        fertilizer_water,
        soil_fertilizer,
        seed_soil,
        seed_ranges,
    ): (
        Option<Vec<(Range<u64>, u64)>>,
        Option<Vec<(Range<u64>, u64)>>,
        Option<Vec<(Range<u64>, u64)>>,
        Option<Vec<(Range<u64>, u64)>>,
        Option<Vec<(Range<u64>, u64)>>,
        Option<Vec<(Range<u64>, u64)>>,
        Option<Vec<(Range<u64>, u64)>>,
        Vec<Range<u64>>,
    ) = std::io::stdin().lines().map(|l| l.unwrap()).fold(
        (None, None, None, None, None, None, None, Vec::new()),
        |mut r, l| {
            if "" == l {
                return r;
            }

            if l.starts_with("seeds:") {
                let seed_ranges = l.split(' ').collect::<Vec<&str>>()[1..]
                    .iter()
                    .map(|v| v.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>()
                    .chunks_exact(2)
                    .into_iter()
                    .map(|range| range[0]..range[0] + range[1])
                    .collect::<Vec<Range<u64>>>();

                r.7 = seed_ranges;

                return r;
            } else if "seed-to-soil map:" == l {
                r.6 = Some(Vec::new());

                return r;
            } else if "soil-to-fertilizer map:" == l {
                r.5 = Some(Vec::new());

                return r;
            } else if "fertilizer-to-water map:" == l {
                r.4 = Some(Vec::new());

                return r;
            } else if "water-to-light map:" == l {
                r.3 = Some(Vec::new());

                return r;
            } else if "light-to-temperature map:" == l {
                r.2 = Some(Vec::new());

                return r;
            } else if "temperature-to-humidity map:" == l {
                r.1 = Some(Vec::new());

                return r;
            } else if "humidity-to-location map:" == l {
                r.0 = Some(Vec::new());

                return r;
            }

            let mut l = l.splitn(3, ' ');

            let destination_start = l.next().unwrap().parse::<u64>().unwrap();
            let source_start = l.next().unwrap().parse::<u64>().unwrap();
            let length = l.next().unwrap().parse::<u64>().unwrap();

            let map = (source_start..source_start + length, destination_start);

            match r {
                (Some(ref mut ms), _, _, _, _, _, _, _) => ms.push(map),
                (_, Some(ref mut ms), _, _, _, _, _, _) => ms.push(map),
                (_, _, Some(ref mut ms), _, _, _, _, _) => ms.push(map),
                (_, _, _, Some(ref mut ms), _, _, _, _) => ms.push(map),
                (_, _, _, _, Some(ref mut ms), _, _, _) => ms.push(map),
                (_, _, _, _, _, Some(ref mut ms), _, _) => ms.push(map),
                (_, _, _, _, _, _, Some(ref mut ms), _) => ms.push(map),
                _ => (),
            }

            r
        },
    );

    let seed_soil = seed_soil.unwrap();
    let soil_fertilizer = soil_fertilizer.unwrap();
    let fertilizer_water = fertilizer_water.unwrap();
    let water_light = water_light.unwrap();
    let light_temperature = light_temperature.unwrap();
    let temperature_humidity = temperature_humidity.unwrap();
    let humidity_location = humidity_location.unwrap();

    let find = |mappings: &Vec<(Range<u64>, u64)>, pos| {
        for (range, destination_start) in mappings {
            if pos >= range.start && pos < range.end {
                return destination_start + (pos - range.start);
            }
        }

        pos
    };

    let result = seed_ranges.into_iter().fold(std::u64::MAX, |r, sr| {
        sr.into_iter().fold(r, |r, s| {
            let soil = find(&seed_soil, s);
            let fertilizer = find(&soil_fertilizer, soil);
            let water = find(&fertilizer_water, fertilizer);
            let light = find(&water_light, water);
            let temperature = find(&light_temperature, light);
            let humidity = find(&temperature_humidity, temperature);
            let location = find(&humidity_location, humidity);

            std::cmp::min(r, location)
        })
    });

    println!("{result}");
}
