use std::collections::HashMap;
use std::time::Duration;
use winit::event_loop::EventLoopProxy;

#[derive(Debug, Clone, Copy)]
pub enum CustomEvent {
    Timer,
    KeyboardInput,
    MouseInput,
    GamepadInput,
}

pub fn spawn_timer_event(duration: Duration, event_loop_proxy: EventLoopProxy<CustomEvent>) -> () {
    std::thread::spawn(move || loop {
        std::thread::sleep(duration);
        event_loop_proxy.send_event(CustomEvent::Timer).unwrap();
    });
}

pub struct ProxyEventLoopManager {
    proxy_loops: HashMap<String, EventLoopProxy<CustomEvent>>
}

impl ProxyEventLoopManager {
    pub fn new() -> Self {
        ProxyEventLoopManager {
            proxy_loops: HashMap::new()
        }
    }

    pub fn add_proxy(&mut self, name: String, proxy: EventLoopProxy<CustomEvent>) {
        self.proxy_loops.insert(name, proxy);
    }

    pub fn get_proxy(&self, name: &str) -> Option<&EventLoopProxy<CustomEvent>> {
        self.proxy_loops.get(name)
    }

    pub fn remove_proxy(&mut self, name: &str) -> Option<EventLoopProxy<CustomEvent>> {
        self.proxy_loops.remove(name)
    }
}