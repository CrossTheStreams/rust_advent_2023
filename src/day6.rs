#[derive(Debug)]
struct BoatRaceRecord {
    time: u32,
    distance: u32
}

pub fn run_day_6_part_1() -> () {
    let records = vec![
        BoatRaceRecord {
            time: 59,
            distance: 597
        },
        BoatRaceRecord {
            time: 79,
            distance: 1234
        },
        BoatRaceRecord {
            time: 65,
            distance: 1032
        }, 
        BoatRaceRecord {
            time: 75,
            distance: 1328
        },
    ];
    let mut ways_to_beat_records: Vec<u32> = vec![];
    for record in &records {
        ways_to_beat_records.push(ways_to_beat_record(record));
    } 
    let answer: u32 = ways_to_beat_records.into_iter().reduce(|acc, e| acc * e).unwrap();
    println!("Part 1: Product of numbers of ways to win races inputs/day6.txt ==> {}", answer)
}

fn ways_to_beat_record(record: &BoatRaceRecord) -> u32 {
    let mut min_milliseconds = 0 as u32;
    let mut distance = 0 as u32;
    // find the minimum number of milliseconds that yields a higher distance
    while distance <= record.distance {
        min_milliseconds += 1;
        distance = (record.time - min_milliseconds) * min_milliseconds
    }

    // find the maximum number of milliseconds that yields a higher distance
    distance = 0;
    let mut max_milliseconds = record.time;
    while distance <= record.distance {
        max_milliseconds -= 1;
        distance = (record.time - max_milliseconds) * max_milliseconds
    }
    return (max_milliseconds - min_milliseconds) + 1;
}

#[test]
fn test_ways_to_beat_record() {
    assert_eq!(
        4,
        ways_to_beat_record(&BoatRaceRecord { time: 7, distance: 9 })
    );
    assert_eq!(
        8,
        ways_to_beat_record(&BoatRaceRecord { time: 15, distance: 40 })
    );
    assert_eq!(
        9,
        ways_to_beat_record(&BoatRaceRecord { time: 30, distance: 200 })
    );
}