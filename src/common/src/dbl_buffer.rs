use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct DoubleBuffer<T> {
    buffers: [Arc<Mutex<Vec<T>>>; 2],
    active: usize,
}

impl<T> DoubleBuffer<T> {
    pub fn new(size: usize) -> Self {
        Self {
            buffers: [
                Arc::new(Mutex::new(Vec::with_capacity(size))),
                Arc::new(Mutex::new(Vec::with_capacity(size))),
            ],
            active: 0,
        }
    }

    pub fn get_active_buffer(&self) -> Arc<Mutex<Vec<T>>> {
        self.buffers[self.active].clone()
    }

    pub fn get_inactive_buffer(&self) -> Arc<Mutex<Vec<T>>> {
        self.buffers[1 - self.active].clone()
    }

    pub fn swap_buffers(&mut self) {
        self.active = 1 - self.active;
    }

    pub fn write_to_inactive_buffer(&self, data: Vec<T>) {
        let binding = self.get_inactive_buffer();
        let mut buffer = binding
            .lock()
            .expect("Failed to lock inactive buffer");

        *buffer = data;
    }
}
#[cfg(test)]
mod tests {
    use crate::dbl_buffer::DoubleBuffer;

    #[test]
    fn test_double_buffer() {
        let mut double_buffer = DoubleBuffer::new(1024);

        // Simulate writing to the inactive buffer
        double_buffer.write_to_inactive_buffer(vec![1, 2, 3, 4]);

        // Swap the buffers
        double_buffer.swap_buffers();

        // Now the previously inactive buffer is active
        let active_buffer = double_buffer.get_active_buffer();
        let buffer_content = active_buffer.lock().unwrap();
        assert_eq!(*buffer_content, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_double_buffer_swap() {
        let mut double_buffer = DoubleBuffer::new(1024);

        // Simulate writing to the inactive buffer
        double_buffer.write_to_inactive_buffer(vec![1, 2, 3, 4]);

        // Swap the buffers
        double_buffer.swap_buffers();

        // Now the previously inactive buffer is active
        let active_buffer = double_buffer.get_active_buffer();
        let buffer_content = active_buffer.lock().unwrap();
        assert_eq!(*buffer_content, vec![1, 2, 3, 4]);

        // Simulate writing to the inactive buffer
        double_buffer.write_to_inactive_buffer(vec![5, 6, 7, 8]);

        // Swap the buffers
        double_buffer.swap_buffers();

        // Now the previously inactive buffer is active
        let active_buffer = double_buffer.get_active_buffer();
        let buffer_content = active_buffer.lock().unwrap();
        assert_eq!(*buffer_content, vec![5, 6, 7, 8]);
    }
}