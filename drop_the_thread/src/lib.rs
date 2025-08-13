use std::cell::{Cell, RefCell};

#[derive(Debug)]
pub struct ThreadPool {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>
}

impl ThreadPool {
    pub fn new() -> Self {
        todo!()
    }

    pub fn new_thread(&self, c: String) -> (usize, Thread) {
        todo!()
    }

    pub fn thread_len(&self) -> usize {
        todo!()
    }

    pub fn is_dropped(&self, id: usize) -> bool {
        todo!()
    }

    pub fn drop_thread(&self, id: usize) {
        todo!()
    }
}

#[derive(Debug)]
pub struct Thread {
    pub pid: usize,
    pub cmd: String,
    pub parent: &'a ThreadPool,
}

impl<'a> Thread<'a> {
    pub fn new(p: usize, c: String, t: &'a ThreadPool) -> Self {
        Thread { pid: p, cmd: c, parent: t }
    }

    pub fn skill(self) {
        drop(self);
    }
}

impl Drop for Thread<'_> {
    self.parent.drop_thread(self.pid);
}