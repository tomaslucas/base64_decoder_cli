pub fn read_file(input_file: &String) -> String {
    std::fs::read_to_string(input_file).unwrap()
}
