pub struct Decoder {}

impl Decoder {
    pub fn decode(line: &str) -> (Range, Range) {
        let ranges = line
            .split(',')
            .map(|range| {
                let mut range = range.split('-');
                Range {
                    start: range.next().unwrap().parse::<i32>().unwrap(),
                    end: range.next().unwrap().parse::<i32>().unwrap(),
                }
            })
            .collect::<Vec<Range>>();

        (ranges[0].clone(), ranges[1].clone())
    }
}

#[derive(Clone)]
pub struct Range {
    start: i32,
    end: i32,
}

impl Range {
    pub fn contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }
    pub fn overlaps(&self, other: &Range) -> bool {
        self.start <= other.end && self.end >= other.start
    }
}
