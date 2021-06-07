struct GapBuffer<T> {
    gap_beg: usize,
    gap_end: usize,
    buffer: Vec<T>,
}

impl<T: Default> GapBuffer<T> {
    fn new(cap: usize) -> Self {
        let mut buffer = Vec::with_capacity(cap);
        for _ in 0..cap {
            buffer.push(T::default());
        }
        Self {
            gap_beg: 0,
            gap_end: cap,
            buffer: buffer,
        }
    }
}


fn main() {
    let _: GapBuffer<i32> = GapBuffer::new(100);
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let buf: GapBuffer<i32> = GapBuffer::new(100);
        assert_eq!(buf.gap_beg, 0);
        assert_eq!(buf.gap_end, 100);
        assert_eq!(buf.buffer, vec![0; 100]);
    }
}
