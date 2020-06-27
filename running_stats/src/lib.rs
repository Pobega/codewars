#[derive(Debug)]
struct Time {
    hr: u32,
    mn: u32,
    sc: u32,
}

type RunnerData = Vec<Time>;

fn parse_runner_data(data: &str) -> RunnerData {
    data.split(',')
        .map(|x| {
            x.split('|')
                .map(|y| {
                    y.split_whitespace()
                        .collect::<String>()
                        .parse::<u32>()
                        .unwrap_or(0)
                })
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}

fn time_to_seconds(data: &Vec<u32>) {
    data
        .fold(0, |acc, x|
}

pub fn stati(strg: &str) -> String {
    let data: RunnerData = parse_runner_data(strg);
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_equals() {
        assert_eq!(
            stati("01|15|59, 1|47|16, 01|17|20, 1|32|34, 2|17|17"),
            "Range: 01|01|18 Average: 01|38|05 Median: 01|32|34"
        );
        assert_eq!(
            stati("02|15|59, 2|47|16, 02|17|20, 2|32|34, 2|17|17, 2|22|00, 2|31|41"),
            "Range: 00|31|17 Average: 02|26|18 Median: 02|22|00"
        );
    }
}
