mod textures;
mod actors;
mod equipment;
mod story_tree;
mod inventory;

use common::dbl_buffer::DoubleBuffer;

fn main() {

    let mut double_buffer = DoubleBuffer::new(1024);

    // Simulate writing to the inactive buffer
    double_buffer.write_to_inactive_buffer(vec![1, 2, 3, 4]);

    // Swap the buffers
    double_buffer.swap_buffers();

    // Now the previously inactive buffer is active
    let active_buffer = double_buffer.get_active_buffer();
    let buffer_content = active_buffer.lock().unwrap();
    println!("{:?}", *buffer_content);
}