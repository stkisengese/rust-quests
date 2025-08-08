pub fn get_diamond(c: char) -> Vec<String> {
    let n = (c as u8 - b'A') as usize;  // 'A' is 0, 'B' is 1, etc.
    let size = 2 * n + 1;  // Total number of lines
    
    (0..size).map(|i| {
        let current_char = (b'A' + (n as i32 - (i as i32 - n as i32).abs()) as u8) as char;
        let left = (i as i32 - n as i32).abs() as usize;
        let right = size - 1 - left;
        let mut line = vec![' '; size];
        line[left] = current_char;
        if left != right {
            line[right] = current_char;
        }
        line.into_iter().collect()
    }).collect()
}