fn main() {
    let input = include_str!("input.txt");

    let lines: Vec<Vec<u32>> = input
    .lines()
    .map(|line| {
        line
        .split(':')
        .last()
        .expect("There should be values after the ':'")
        .split_whitespace().map(|num| num.parse::<u32>().expect("Should be a number"))
        .collect()
    })
    .collect();

    let result: u32 = 
    lines[0]
    .iter()
    .zip(lines[1].iter())
    .map(|(a, b)| {
        let race = Race{time: *a, record_distance: *b};
        race
    })
    .map(|race| race_to_winning_ways(race))
    .product();

    dbg!(result);
}

#[derive(Debug)]
struct Race {
    time: u32,
    record_distance: u32,
}

fn pressed_to_distance_over_time(time_pressed: u32, time_max: u32) -> u32 {
    let speed = time_pressed;
    let remaining_time = time_max - time_pressed;

    speed * remaining_time
}

fn race_to_winning_ways(race: Race) -> u32 {
    (0..race.time).map(|potential_time| {
        pressed_to_distance_over_time(potential_time, race.time)
    }).filter(|&distance| distance > race.record_distance).count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pressing_0_ms_returns_0_distance() {
        assert_eq!(pressed_to_distance_over_time(0, 1000), 0);
    }

    #[test]
    fn pressing_1_ms_returns_remaining_time_as_distance() {
        let total_time = 5;
        let time_pressed = 1;
        let expected_distance = total_time - time_pressed;

        assert_eq!(pressed_to_distance_over_time(time_pressed, total_time), expected_distance);
    }

    #[test]
    fn pressing_2ms_gives_distance_of_10_milimeters() {
        let total_time = 7;
        let time_pressed = 2;
        let expected_distance = 10;

        assert_eq!(pressed_to_distance_over_time(time_pressed, total_time), expected_distance);
    }

    #[test]
    fn pressing_7_ms_gives_distance_of_0_milimeters() {
        let total_time = 7;
        let time_pressed = 7;
        let expected_distance = 0;

        assert_eq!(pressed_to_distance_over_time(time_pressed, total_time), expected_distance);
    }

    #[test]
    fn correct_number_of_winning_ways_for_7ms_example() {
        let race = Race {time: 7, record_distance: 9};
        let expected_winning_ways = 4;

        assert_eq!(race_to_winning_ways(race), expected_winning_ways);
    }

}