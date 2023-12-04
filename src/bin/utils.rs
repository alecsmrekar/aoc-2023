use std::fs;

pub fn get_lines(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string("src/input/".to_string() + filename)
        .expect("Should have been able to read the file");
    let lines = contents
        .split("\n")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    return lines;
}

pub fn get_lines_nonempty(filename: &str) -> Vec<String> {
    return get_lines(filename)
        .iter()
        .filter(|line| !line.is_empty())
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
}
