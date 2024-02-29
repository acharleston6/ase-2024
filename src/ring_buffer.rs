pub struct RingBuffer<T> {
    buffer: Vec<T>,
    head: usize,
    tail: usize,
}

impl<T: Copy + Default> RingBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        RingBuffer {
            buffer: vec![T::default(); capacity],
            head: 0,
            tail: 0,
        }
    }

    pub fn reset(&mut self) {
        self.buffer.fill(T::default());
        self.head = 0;
        self.tail = 0;
    }

    // `put` and `peek` write/read without advancing the indices.
    pub fn put(&mut self, value: T) {
        self.buffer[self.head] = value
    }

    pub fn peek(&self) -> T {
        self.buffer[self.tail]
    }

    pub fn get(&self, offset: usize) -> T {
        self.buffer[(self.tail + offset) % self.capacity()]
    }

    // `push` and `pop` write/read and advance the indices.
    pub fn push(&mut self, value: T) {
        self.buffer[self.head] = value;
        self.head = (self.head + 1) % self.capacity();
    }

    pub fn pop(&mut self) -> T {
        let value = self.buffer[self.tail];
        self.tail = (self.tail + 1) % self.capacity();
        value
    }

    pub fn get_read_index(&self) -> usize {
        self.tail
    }

    pub fn set_read_index(&mut self, index: usize) {
        self.tail = index % self.capacity()
    }

    pub fn get_write_index(&self) -> usize {
        self.head
    }

    pub fn set_write_index(&mut self, index: usize) {
        self.head = index % self.capacity()
    }

    pub fn len(&self) -> usize {
        // Return number of values currently in the ring buffer.
        if self.head >= self.tail {
            self.head - self.tail
        } else {
            self.head + self.capacity() - self.tail
        }
    }

    pub fn capacity(&self) -> usize {
        // Return the size of the internal buffer.
        self.buffer.len()
    }
}

impl RingBuffer<f32> {
    // Return the value at at an offset from the current read index.
    // To handle fractional offsets, linearly interpolate between adjacent values. 
    pub fn get_frac(&self, offset: f32) -> f32 {  // 0.5
        let currFloor = self.tail as f32 + offset.floor();      // 0.0
        let currCeil = self.tail as f32 + offset.ceil();        // 1.0
        //println!("floor: {}, ceil: {}", currFloor, currCeil);

        // ADD EXCEPTION FOR IF FLOOR AND CEIL ARE SAME

        let left =  self.buffer[(currFloor as usize) % self.capacity()];            // 1.0
        let right = self.buffer[(currCeil as usize)  % self.capacity()];            // 2.0
        //println!("left: {}, right: {}", left, right);

        let leftPercent  = 1.0 - (offset - offset.floor());     // .5
        let rightPercent = 0.0 + (offset - offset.floor());     // .5
        //println!("leftPercent: {}, rightPercent: {}", leftPercent, rightPercent);

        let output = left * leftPercent + right * rightPercent; // 1.5
        return output;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrapping() {
        // Test that ring buffer is a ring (wraps after more than `length` elements have entered).
        let capacity = 17;
        let delay = 5;
        let mut ring_buffer: RingBuffer<f32> = RingBuffer::new(capacity);

        for i in 0..delay {
            ring_buffer.push(i as f32);
        }

        for i in delay..capacity + 13 {
            assert_eq!(ring_buffer.len(), delay);
            assert_eq!(ring_buffer.pop(), (i - delay) as f32);
            ring_buffer.push(i as f32)
        }
    }

    #[test]
    fn test_api() {
        // Basic test of all API functions.
        let capacity = 3;
        let mut ring_buffer = RingBuffer::new(capacity);
        assert_eq!(ring_buffer.capacity(), capacity);

        ring_buffer.put(3);
        assert_eq!(ring_buffer.peek(), 3);

        ring_buffer.set_write_index(1);
        assert_eq!(ring_buffer.get_write_index(), 1);

        ring_buffer.push(17);
        assert_eq!(ring_buffer.get_write_index(), 2);

        assert_eq!(ring_buffer.get_read_index(), 0);
        assert_eq!(ring_buffer.get(1), 17);
        assert_eq!(ring_buffer.pop(), 3);
        assert_eq!(ring_buffer.get_read_index(), 1);

        assert_eq!(ring_buffer.len(), 1);
        ring_buffer.push(42);
        assert_eq!(ring_buffer.len(), 2);

        assert_eq!(ring_buffer.get_write_index(), 0);

        // Should be unchanged.
        assert_eq!(ring_buffer.capacity(), capacity);
    }

    #[test]
    fn test_capacity() {
        // Tricky: does `capacity` mean "size of internal buffer" or "number of elements before this is full"?
        let capacity = 3;
        let mut ring_buffer = RingBuffer::new(3);
        for i in 0..(capacity - 1) {
            ring_buffer.push(i);
            dbg!(ring_buffer.len());
            assert_eq!(ring_buffer.len(), i+1);
        }
    }

    #[test]
    fn test_reset() {
        // Test state after initialization and reset.
        let mut ring_buffer = RingBuffer::new(512);

        // Check initial state.
        assert_eq!(ring_buffer.get_read_index(), 0);
        assert_eq!(ring_buffer.get_write_index(), 0);
        for i in 0..ring_buffer.capacity() {
            assert_eq!(ring_buffer.get(i), 0.0);
        }

        // Fill ring buffer, mess with indices.
        let fill = 123.456;
        for i in 0..ring_buffer.capacity() {
            ring_buffer.push(fill);
            assert_eq!(ring_buffer.get(i), fill);
        }

        ring_buffer.set_write_index(17);
        ring_buffer.set_read_index(42);

        // Check state after reset.
        ring_buffer.reset();
        assert_eq!(ring_buffer.get_read_index(), 0);
        assert_eq!(ring_buffer.get_write_index(), 0);
        for i in 0..ring_buffer.capacity() {
            assert_eq!(ring_buffer.get(i), 0.0);
        }
    }

    #[test]
    fn test_weird_inputs() {
        let capacity = 5;
        let mut ring_buffer = RingBuffer::<f32>::new(capacity);

        ring_buffer.set_write_index(capacity);
        assert_eq!(ring_buffer.get_write_index(), 0);
        ring_buffer.set_write_index(capacity * 2 + 3);
        assert_eq!(ring_buffer.get_write_index(), 3);

        ring_buffer.set_read_index(capacity);
        assert_eq!(ring_buffer.get_read_index(), 0);
        ring_buffer.set_read_index(capacity * 2 + 3);
        assert_eq!(ring_buffer.get_read_index(), 3);

        // NOTE: Negative indices are also weird, but we can't even pass them due to type checking!
    }

    #[test]
    fn test_ringbuffer_f32() {
        let capacity = 5;
        let mut ring_buffer = RingBuffer::<f32>::new(capacity);
        ring_buffer.push(1.0);
        ring_buffer.push(2.0);    
        let x = ring_buffer.get_frac(0.5);
        //println!("x: {}", x);
        assert_eq!(x, 1.5); 

        ring_buffer.push(100.0);
        ring_buffer.push(10.0);   
        let y = ring_buffer.get_frac(2.1);
        //println!("y: {}", y);
        assert_eq!(y, 91.00001);    
        // y - expected.abs() < ESPILSO         
    }

}
