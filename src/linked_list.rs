use element::Element;

#[derive(Clone)]
pub struct LinkedList {
    head: Option<Box<Element>>,
    tail: Option<Box<Element>>,
    size: u32
}

impl LinkedList {
    pub fn new() -> LinkedList {
        LinkedList {
            head: None,
            tail: None,
            size: 0
        }
    }

    pub fn get_head(&self) -> Option<Box<Element>> {
        self.head.to_owned()
    }

    pub fn set_head(&mut self, head: Element) {
        self.head = Some(Box::new(head));
    }

    pub fn get_tail(&self) -> Option<Box<Element>> {
        self.tail.to_owned()
    }

    pub fn set_tail(&mut self, tail: Element) {
        self.tail = Some(Box::new(tail));
    }
}