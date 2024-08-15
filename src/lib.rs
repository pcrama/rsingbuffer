use std::mem::swap;

pub struct RingBuffer<A> {
    buffer: Vec<A>,
    start: usize,
    end: usize,
    capacity: usize,
}

impl<A> RingBuffer<A> {
    pub fn new(size: usize) -> RingBuffer<A> {
        assert!(size > 0);
        RingBuffer {
            buffer: Vec::<A>::with_capacity(size),
            start: 0,
            end: 0,
            capacity: size,
        }
    }

    pub fn len(&self) -> usize {
        if self.start == 0 && self.end == 0 {
            return 0;
        } else if self.start <= self.end {
            return self.end - self.start;
        } else {
            return self.buffer.len() + self.end - self.start;
        }
    }

    pub fn peek_first<B>(&self, cont: fn(&A) -> B) -> Option<B> {
        if self.start == 0 && self.end == 0 {
            return None;
        } else {
            return Some(cont(&self.buffer[self.start]));
        }
    }

    pub fn peek_last<B>(&self, cont: fn(&A) -> B) -> Option<B> {
        if self.start == 0 && self.end == 0 {
            return None;
        } else {
            return Some(cont(&self.buffer[self.end - 1]));
        }
    }

    pub fn push(&mut self, val: A) -> Option<A> {
        if self.start == 0 {
            if self.end >= self.capacity {
                let mut val = val;
                swap(&mut self.buffer[0], &mut val);
                self.start = 1;
                self.end = 1;
                return Some(val);
            } else {
                if self.buffer.len() < self.buffer.capacity() {
                    self.buffer.push(val);
                } else {
                    self.buffer[self.end] = val;
                }
                self.end += 1;
                return None;
            }
        } else if self.start == self.end {
            let mut val = val;
            swap(&mut self.buffer[self.end], &mut val);
            self.end += 1;
            if self.end < self.capacity {
                self.start = self.end;
            } else {
                self.start = 0;
            }
            return Some(val);
        } else {
            self.buffer[self.end] = val;
            self.end += 1;
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fresh_ringbuffer_len_is_0() {
        assert_eq!(RingBuffer::<&str>::new(5).len(), 0);
    }

    fn idint(x: &i32) -> i32 {
        *x
    }

    #[test]
    fn fresh_ringbuffer_peek_is_none() {
        assert_eq!(RingBuffer::<i32>::new(23).peek_first(idint), None);
        assert_eq!(RingBuffer::<i32>::new(3).peek_last(idint), None);
    }

    #[test]
    fn fresh_ringbuffer_peek_when_filling() {
        let mut rb = RingBuffer::<i32>::new(3);
        rb.push(3);
        assert_eq!(rb.peek_first(idint), Some(3));
        assert_eq!(rb.peek_last(idint), Some(3));
        rb.push(4);
        assert_eq!(rb.peek_first(idint), Some(3));
        assert_eq!(rb.peek_last(idint), Some(4));
        rb.push(5);
        assert_eq!(rb.peek_first(idint), Some(3));
        assert_eq!(rb.peek_last(idint), Some(5));
        rb.push(6);
        assert_eq!(rb.peek_first(idint), Some(4));
        assert_eq!(rb.peek_last(idint), Some(6));
        rb.push(7);
        assert_eq!(rb.peek_first(idint), Some(5));
        assert_eq!(rb.peek_last(idint), Some(7));
        rb.push(8);
        assert_eq!(rb.peek_first(idint), Some(6));
        assert_eq!(rb.peek_last(idint), Some(8));
        rb.push(9);
        assert_eq!(rb.peek_first(idint), Some(7));
        assert_eq!(rb.peek_last(idint), Some(9));
    }

    #[test]
    fn ringbuffer_overwrites_when_pushing_enough() {
        let mut rb = RingBuffer::<i32>::new(3);
        assert_eq!(rb.push(3), None);
        assert_eq!(rb.push(4), None);
        assert_eq!(rb.push(5), None);
        assert_eq!(rb.push(6), Some(3));
        assert_eq!(rb.push(7), Some(4));
    }
}
