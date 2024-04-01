#[test]
fn start_of_pack_sample1() {
    const PATH: &str = "src/day6/sample1.txt";
    let data = std::fs::read_to_string(PATH).unwrap();
    assert_eq!(super::start_of_pack(&data, 14), 19);
}

#[test]
fn start_of_pack_sample2() {
    const PATH: &str = "src/day6/sample2.txt";
    let data = std::fs::read_to_string(PATH).unwrap();
    assert_eq!(super::start_of_pack(&data, 14), 23);
}

#[test]
fn start_of_pack_sample3() {
    const PATH: &str = "src/day6/sample3.txt";
    let data = std::fs::read_to_string(PATH).unwrap();
    assert_eq!(super::start_of_pack(&data, 14), 23);
}

#[test]
fn start_of_pack_sample4() {
    const PATH: &str = "src/day6/sample4.txt";
    let data = std::fs::read_to_string(PATH).unwrap();
    assert_eq!(super::start_of_pack(&data, 14), 29);
}

#[test]
fn start_of_pack_sample5() {
    const PATH: &str = "src/day6/sample5.txt";
    let data = std::fs::read_to_string(PATH).unwrap();
    assert_eq!(super::start_of_pack(&data, 14), 26);
}
