use linked_list::LinkedList;
use element::Element;

#[test]
fn create_list() {
    LinkedList::new();
}

#[test]
fn create_element() {
    let element = Element::new("hello".to_string());
    assert_eq!(element.get_value(), "hello");
}

#[test]
fn set_value() {
    let mut element = Element::new("".to_string());
    assert_eq!(element.get_value(), "");
    &mut element.set_value("changed".to_string());
    assert_eq!(element.get_value(), "changed");
}

#[test]
fn get_elements() {
    let element = Element::new("hello".to_string());
    assert_eq!(element.get_next().is_none(), true);
    assert_eq!(element.get_prev().is_none(), true);
}