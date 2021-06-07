struct GapBuffer<T> {
    gap_beg: usize,
    gap_end: usize,
    buffer: Vec<T>,
}

// TODO: research about uninitialized value handling and remove Clone trait constraint from T
impl<T: Clone> GapBuffer<T> {
    fn new(zero: T, cap: usize) -> Self {
        Self {
            gap_beg: 0,
            gap_end: cap,
            buffer: vec![zero; cap],
        }
    }
}


fn main() {
    let _ = GapBuffer::new(0, 100);
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let buf = GapBuffer::new(0, 100);
        assert_eq!(buf.gap_beg, 0);
        assert_eq!(buf.gap_end, 100);
        assert_eq!(buf.buffer, vec![0; 100]);
    }
}
