#[derive(Clone, Debug, Default)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T:Default> List<T> {
    pub fn new() -> List<T> {
        List::default()
    }

    pub fn push(&mut self, value: T) {
        let new_node: Node<T> = Node {
            value,
            next: self.head.take().map(Box::new),
        };
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) {
        if let Some(node) = self.head.take() {
            self.head = node.next.map(|boxed_node: Box<Node<T>> | *boxed_node);
        };
    }

    pub fn len(&self) -> usize {
        let mut count = 0 as usize;
        if let Some(head) = &self.head {
            let mut current = Some(head);
            while let Some(node) = current {
                count += 1;
                current = node.next.as_ref().map(|boxed_node| &**boxed_node);
                if node.next.is_none() {
                    break;
                }
            }
        };
        count
    }
}
