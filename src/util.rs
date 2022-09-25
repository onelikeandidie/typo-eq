pub fn get_index(vector: &Vec<String>, key: &str) -> i32 {
    let lookup = vector.iter().position(|v| v == key);
    match lookup {
        Some(index) => return index as i32,
        None => return -1,
    }
}

pub fn get_index_of_line(txt: &str, index: usize) -> usize {
    let lines_slice = &txt.split("\n").collect::<Vec<&str>>()[0..index];
    let lines = lines_slice
        .into_iter()
        .map(|line| line.len())
        .collect::<Vec<usize>>();
    // Count chars of lines before index
    let result_index = lines
        .into_iter()
        .reduce(|accum, line_len| {
            // Don't forget the \n char
            return accum + line_len + 1;
        })
        .unwrap();
    return result_index;
}
