use std::cell::{RefCell};
use std::rc::{Rc};
use std::collections::{VecDeque};

type Queue<T> = Rc<RefCell<VecDeque<T>>>;

pub struct Bus<T> {
    queues: Vec<Queue<T>>,
}

impl<T: Clone> Bus<T> {
    pub fn new() -> Self {
        Bus {
            queues: Vec::new(),
        }
    }

    pub fn add_rx(&mut self) -> BusReader<T> {
        let queue = Queue::default();
        self.queues.push(queue.clone());
        BusReader {
            queue,
        }
    }

    pub fn broadcast(&self, value: &T) {
        for queue in &self.queues {
            queue.borrow_mut().push_back(value.clone());
        }
    }
}

pub struct BusReader<T> {
    queue: Queue<T>,
}

impl<T> BusReader<T> {
    pub fn try_recv(&self) -> Option<T> {
        self.queue.borrow_mut().pop_front()
    }
}
