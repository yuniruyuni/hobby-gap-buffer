use std::mem::take;

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

    fn cap(&self) -> usize {
        self.buffer.len()
    }

    fn gap_cap(&self) -> usize {
        self.gap_end - self.gap_beg
    }

    fn len(&self) -> usize {
        self.cap() - self.gap_cap()
    }

    fn set_cursor(&mut self, pos: usize) {
        assert!(pos <= self.len());

        let gap_beg = self.gap_beg;
        let gap_end = self.gap_end;
        let gap_cap = self.gap_cap();

        let copy = |buf: &mut Vec<T>, src_beg, dst_beg, len| {
            for i in 0..len {
                let src = src_beg + i;
                let dst = dst_beg + i;
                buf[dst] = take(&mut buf[src]);
            }
        };

        let revcopy = |buf: &mut Vec<T>, src_end, dst_end, len| {
            for i in 1..=len {
                let src = src_end - i;
                let dst = dst_end - i;
                buf[dst] = take(&mut buf[src]);
            }
        };

        if pos < gap_beg {
            // copy [pos, gap_beg] to [gap_end - (pos - gap_beg), gap_end]
            let cpy_len = gap_beg - pos;
            let src_end = gap_beg;
            let dst_end = gap_end;
            revcopy(&mut self.buffer, src_end, dst_end, cpy_len);
        } else if gap_beg <= pos && pos < gap_end {
            // copy [gap_end - (pos - gap_beg), gap_end] to [gap_beg, pos]
            let cpy_len = pos - gap_beg;
            let src_beg = gap_end - cpy_len;
            let dst_beg = gap_beg - cpy_len;
            copy(&mut self.buffer, src_beg, dst_beg, cpy_len);
        } else {
            // copy [gap_end - (pos - gap_beg), gap_end] to [gap_beg, pos]
            let cpy_len = pos - gap_beg;
            let src_beg = gap_end;
            let dst_beg = gap_beg;
            copy(&mut self.buffer, src_beg, dst_beg, cpy_len);
        }

        self.gap_beg = pos;
        self.gap_end = pos + gap_cap;
    }

    fn insert(&mut self, t: T) {
        self.extend_gap();

        self.buffer[self.gap_beg] = t;
        self.gap_beg += 1;
    }

    fn extend_gap(&mut self) {
        if self.gap_beg < self.gap_end {
            return;
        }

        let cur_len = self.buffer.len();
        let new_len = cur_len * 2;
        let mut newbuf = Vec::with_capacity(new_len);

        for i in 0..self.gap_beg {
            newbuf.push(take(&mut self.buffer[i]));
        }

        for _ in cur_len..new_len {
            newbuf.push(T::default());
        }

        for i in self.gap_end..cur_len {
            newbuf.push(take(&mut self.buffer[i]));
        }

        self.buffer = newbuf;
        self.gap_end = self.gap_beg + (new_len - cur_len);
    }
}


fn main() {
    let mut buf: GapBuffer<i32> = GapBuffer::new(2);
    buf.insert( 1 );
    buf.insert( 2 );
    buf.insert( 3 );
    buf.set_cursor( 1 );
    buf.insert( 4 );
    println!("{:?}", buf.buffer);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let buf: GapBuffer<i32> = GapBuffer::new(100);
        assert_eq!(buf.gap_beg, 0);
        assert_eq!(buf.gap_end, 100);
        assert_eq!(buf.gap_cap(), 100);
        assert_eq!(buf.cap(), 100);
        assert_eq!(buf.len(), 0);
        assert_eq!(buf.buffer, vec![0; 100]);
    }

    #[test]
    fn insert() {
        let mut buf: GapBuffer<i32> = GapBuffer::new(2);
        buf.insert(1);
        assert_eq!(buf.gap_beg, 1);
        assert_eq!(buf.gap_end, 2);
        assert_eq!(buf.buffer, vec![1, 0]);
    }

    #[test]
    fn insert_extend() {
        let mut buf: GapBuffer<i32> = GapBuffer::new(2);
        buf.insert(1);
        buf.insert(2);
        buf.insert(3);
        assert_eq!(buf.gap_beg, 3);
        assert_eq!(buf.gap_end, 4);
        assert_eq!(buf.buffer, vec![1, 2, 3, 0]);
    }

    #[test]
    fn set_cursor() {
        let mut buf: GapBuffer<i32> = GapBuffer::new(2);
        buf.insert(1);
        buf.set_cursor(0);
        assert_eq!(buf.gap_beg, 0);
        assert_eq!(buf.gap_end, 1);
        assert_eq!(buf.buffer, vec![0, 1]);
    }


    #[test]
    fn set_cursor_pos_lt_beg() {
        let mut buf: GapBuffer<i32> = GapBuffer::new(4);
        buf.insert(1);
        buf.insert(2);
        buf.insert(3);
        assert_eq!(buf.gap_beg, 3);
        assert_eq!(buf.gap_end, 4);
        assert_eq!(buf.buffer, vec![1, 2, 3, 0]);
        buf.set_cursor(1);
        assert_eq!(buf.gap_beg, 1);
        assert_eq!(buf.gap_end, 2);
        assert_eq!(buf.buffer, vec![1, 0, 2, 3]);
    }

    #[test]
    fn set_cursor_pos_eq_beg() {
        let mut buf: GapBuffer<i32> = GapBuffer::new(2);
        buf.insert(1);
        buf.set_cursor(1);
        assert_eq!(buf.gap_beg, 1);
        assert_eq!(buf.gap_end, 2);
        assert_eq!(buf.buffer, vec![1, 0]);
    }

    #[test]
    fn set_cursor_pos_eq_end() {
        let mut buf: GapBuffer<i32> = GapBuffer::new(4);
        buf.insert(1);
        buf.insert(2);
        buf.set_cursor(0);
        assert_eq!(buf.gap_beg, 0);
        assert_eq!(buf.gap_end, 2);
        buf.set_cursor(2);
        assert_eq!(buf.gap_beg, 2);
        assert_eq!(buf.gap_end, 4);
        assert_eq!(buf.buffer, vec![1, 2, 0, 0]);
    }

    #[test]
    fn set_cursor_pos_gt_end() {
        let mut buf: GapBuffer<i32> = GapBuffer::new(4);
        buf.insert(1);
        buf.insert(2);
        buf.insert(3);
        assert_eq!(buf.gap_beg, 3);
        assert_eq!(buf.gap_end, 4);
        assert_eq!(buf.buffer, vec![1, 2, 3, 0]);
        buf.set_cursor(1);
        assert_eq!(buf.gap_beg, 1);
        assert_eq!(buf.gap_end, 2);
        assert_eq!(buf.buffer, vec![1, 0, 2, 3]);
        buf.set_cursor(3);
        assert_eq!(buf.gap_beg, 3);
        assert_eq!(buf.gap_end, 4);
        assert_eq!(buf.buffer, vec![1, 2, 3, 0]);
    }
}
