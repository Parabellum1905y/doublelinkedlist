#[derive(Clone)]
pub struct Element {
    value: String,
    next: Option<Box<Element>>,
    prev: Option<Box<Element>>
}

impl Element {
    pub fn new(value: String) -> Element {
        Element {
            value: value,
            next: None,
            prev: None
        }
    }

    pub fn from_existing(value: String, prev: Option<Box<Element>>, next: Option<Box<Element>>) -> Element {
        Element {
            value: value,
            next: next,
            prev: prev
        }
    }

    pub fn get_next(&self) -> Option<Box<Element>> {
        self.next.to_owned()
    }

    pub fn set_next(&mut self, element: Element) {
        self.next = Some(Box::new(element));
    }

    pub fn get_prev(&self) -> Option<Box<Element>> {
        self.prev.to_owned()
    }

    pub fn set_prev(&mut self, element: Element) {
        self.prev = Some(Box::new(element));
    }

    pub fn get_value(&self) -> String {
        self.value.to_owned()
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }
}