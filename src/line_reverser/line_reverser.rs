pub fn reverse_file_lines(buffer: Vec<u8>) -> Vec<u8> {
    // Split the buffer into lines while keeping line endings intact
    // let mut lines: Vec<&[u8]> = buffer.split_inclusive(|&b| b == b'\n').collect();
    let mut lines = split_buffer_into_lines(&buffer);
    lines.reverse();
    lines.concat()
}

fn split_buffer_into_lines(buffer: &Vec<u8>) -> Vec<&[u8]> {
    buffer.split_inclusive(|&b| b == b'\n').collect()
}
