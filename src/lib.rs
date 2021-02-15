// #![feature(test)]

#[cfg(test)]
mod tests;

struct RingBuffer<T> {
    buf: Vec<Option<T>>,
    head: usize,
    tail: usize,
}

impl<T> RingBuffer<T> {
    fn with_capacity(n: usize) -> RingBuffer<T> {
        return RingBuffer {
            buf: Vec::with_capacity(n),
            head: 0,
            tail: 0,
        };
    }
    fn push(&mut self, x: T) {
        self.handle_len();
        if self.is_full() {
            self.grow();
        }
        self.buf[self.head] = Some(x);
        self.head = (self.head + 1) % self.cap()
    }
    fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let popped = self.buf[self.tail].take();
        self.tail = (self.tail + 1) % self.cap();
        return popped;
    }

    fn handle_len(&mut self) {
        self.buf.resize_with(self.cap(), || None)
    }

    fn handle_capacity_increase(&mut self, old_capacity: usize) {
        let new_capacity = self.cap();
        if self.tail <= self.head {
            //    T             H
            //   [o o o o o o o . ]
            //    T             H
            //   [o o o o o o o . . . . . . . . . ]
            // Nop
        } else if self.head < old_capacity - self.tail {
            //        H T
            //   [o o . o o o o o ]
            //          T             H
            //   [. . . o o o o o o o . . . . . . ]
            for i in 0..self.head {
                self.buf[old_capacity + i] = self.buf[i].take()
            }
            self.head += old_capacity;
            debug_assert!(self.head > self.tail);
        } else {
            //              H T
            //   [o o o o o . o o ]
            //              H                 T
            //   [o o o o o . . . . . . . . . o o ]
            let new_tail = new_capacity - (old_capacity - self.tail);
            for i in 0..old_capacity - self.tail {
                self.buf[new_tail + i] = self.buf[self.tail + i].take()
            }
            self.tail = new_tail;
            debug_assert!(self.head < self.tail);
        }
        debug_assert!(self.head < self.cap());
        debug_assert!(self.tail < self.cap());
    }
    fn is_empty(&self) -> bool {
        self.head == self.tail
    }

    fn is_full(&self) -> bool {
        self.cap() as i64 - self.len() as i64 <= 1
    }
    fn len(&self) -> usize {
        if self.cap() == 0 {
            return 0;
        }
        ((self.head as i64 - self.tail as i64 + self.cap() as i64) % self.cap() as i64) as usize
    }
    fn cap(&self) -> usize {
        self.buf.capacity()
    }
    fn grow(&mut self) {
        let mut old_capacity = self.cap();
        if old_capacity == 0 {
            old_capacity = 1
        }
        self.buf.reserve(old_capacity);
        self.handle_len();
        self.handle_capacity_increase(old_capacity);
    }
}

impl<T> IntoIterator for RingBuffer<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { inner: self }
    }
}

pub struct IntoIter<T> {
    inner: RingBuffer<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.inner.pop()
    }
}
