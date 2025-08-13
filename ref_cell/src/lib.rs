use std::cell::RefCell;
use std::rc::Rc;

pub mod messenger;

pub struct Tracker {
    pub messages: RefCell<Vec<String>>,
    value: RefCell<usize>,
    max: usize,
}

impl Tracker {
    pub fn new(max: usize) -> Tracker {
        Tracker {
            messages: RefCell::new(Vec::new()),
            value: RefCell::new(0),
            max,
        }
    }

    pub fn set_value<T>(&self, value: &Rc<T>) {
        let ref_count = Rc::strong_count(value);
        
        // Check if it exceeds max
        if ref_count > self.max {
            self.messages.borrow_mut().push("Error: You can't go over your quota!".to_string());
            return;
        }
        
        // Update the tracker's value
        *self.value.borrow_mut() = ref_count;
        
        // Check if it exceeds 70% but doesn't exceed max
        let percentage = (ref_count * 100) / self.max;
        if percentage > 70 {
            self.messages.borrow_mut().push(format!("Warning: You have used up over {}% of your quota!", percentage));
        }
    }

    pub fn peek<T>(&self, value: &Rc<T>) {
        let ref_count = Rc::strong_count(value);
        let percentage = (ref_count * 100) / self.max;
        self.messages.borrow_mut().push(format!("Info: This value would use {}% of your quota", percentage));
    }
}