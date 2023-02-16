use crate::List::*;

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    fn add(self, e: u32) -> List {
        Cons(e, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}. {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            }
        }
    }
}

fn main() {
    let mut lst = List::new();
    println!("List before additions: {}, length equals {}", lst.stringify(), lst.len());
    lst = lst.add(4);
    lst = lst.add(2);
    lst = lst.add(5);
    println!("List after additions: {}, length equals {}", lst.stringify(), lst.len());
}
