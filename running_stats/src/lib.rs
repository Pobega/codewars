#[derive(Debug, PartialEq)]
struct Time {
    h: u32,
    m: u32,
    s: u32,
}

impl From<Vec<u32>> for Time {
    fn from(item: Vec<u32>) -> Self {
        Time {
            h: item[0],
            m: item[1],
            s: item[2],
        }
    }
}

impl From<u32> for Time {
    fn from(item: u32) -> Self {
        Time {
            h: item / 3600,
            m: item % 3600 / 60,
            s: item % 3600 % 60 / 60,
        }
    }
}

type Seconds = u32;
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
                .into()
        })
        .collect::<RunnerData>()
}

pub fn stati(strg: &str) -> String {
    if strg == "" {
        return "".to_string();
    }
    let data: RunnerData = parse_runner_data(strg);
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_data() {
        let mut runner_data: RunnerData = RunnerData::new();
        runner_data.push(Time { h: 1, m: 1, s: 1 });
        runner_data.push(Time { h: 2, m: 2, s: 2 });
        runner_data.push(Time { h: 3, m: 3, s: 3 });

        assert_eq!(parse_runner_data("01|01|01, 2|02|2, 3|03|3"), runner_data);
        // Handle too many gracefully (ignore extra data)
        assert_eq!(
            parse_runner_data("01|01|01|02, 2|02|2, 3|03|3"),
            runner_data
        );
    }

    #[test]
    #[should_panic]
    fn bad_data() {
        let mut runner_data: RunnerData = RunnerData::new();
        runner_data.push(Time { h: 1, m: 1, s: 1 });
        runner_data.push(Time { h: 2, m: 2, s: 2 });
        runner_data.push(Time { h: 3, m: 3, s: 3 });
        // Handle too few by panicing
        assert_eq!(parse_runner_data("01|01, 2|02|2, 3|03|3"), runner_data);
    }

    #[test]
    fn empty_data() {
        assert_eq!(stati(""), "".to_string());
    }

    /*
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
    */
}
