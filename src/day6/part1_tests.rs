#[test]
fn start_of_pack_sample1_part1() {
    const PATH: &str = "src/day6/sample1.txt";
    let data = std::fs::read_to_string(PATH).unwrap();
    assert_eq!(super::start_of_pack(&data, 4), 7);
}

#[test]
fn start_of_pack_sample2_part1() {
    const PATH: &str = "src/day6/sample2.txt";
    let data = std::fs::read_to_string(PATH).unwrap();
    assert_eq!(super::start_of_pack(&data, 4), 5);
}

#[test]
fn start_of_pack_sample3_part1() {
    const PATH: &str = "src/day6/sample3.txt";
    let data = std::fs::read_to_string(PATH).unwrap();
    assert_eq!(super::start_of_pack(&data, 4), 6);
}

#[test]
fn start_of_pack_sample4_part1() {
    const PATH: &str = "src/day6/sample4.txt";
    let data = std::fs::read_to_string(PATH).unwrap();
    assert_eq!(super::start_of_pack(&data, 4), 10);
}

#[test]
fn start_of_pack_sample5_part1() {
    const PATH: &str = "src/day6/sample5.txt";
    let data = std::fs::read_to_string(PATH).unwrap();
    assert_eq!(super::start_of_pack(&data, 4), 11);
}
