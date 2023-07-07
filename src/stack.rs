#[derive(Debug, Clone)]
enum SNode<T>
where
    T: Clone + std::fmt::Debug,
{
    Base { value: T },
    Body { value: T, prev: *mut SNode<T> },
}

#[derive(Debug, Clone)]
enum Head<T>
where
    T: Clone + std::fmt::Debug,
{
    None,
    One { value: T },
    Some { value: T, prev: *mut SNode<T> },
}

#[derive(Debug, Clone)]
pub struct Stack<T>
where
    T: Clone + std::fmt::Debug,
{
    length: usize,
    head: Head<T>,
}

impl<T: Clone + std::fmt::Debug> Drop for Stack<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

impl<T: Clone + std::fmt::Debug> Stack<T> {
    pub const fn new() -> Self {
        Self {
            length: 0,
            head: Head::None,
        }
    }

    pub fn push(&mut self, item: T) {
        self.length += 1;
        match &self.head {
            Head::None => {
                self.head = Head::One { value: item };
            }

            Head::One { value } => {
                self.head = Head::Some {
                    value: item,
                    prev: Box::into_raw(Box::new(SNode::Base {
                        value: value.clone(),
                    })),
                }
            }

            Head::Some { value, prev } => {
                self.head = Head::Some {
                    value: item,
                    prev: Box::into_raw(Box::new(SNode::Body {
                        value: value.clone(),
                        prev: *prev,
                    })),
                }
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match &self.head {
            Head::None => None,
            Head::One { value } => {
                let value = value.clone();
                self.head = Head::None;
                self.length -= 1;
                Some(value)
            }
            Head::Some { value, prev } => {
                let value = value.clone();
                self.length -= 1;
                unsafe {
                    match *Box::from_raw(*prev) {
                        SNode::Base { value } => self.head = Head::One { value },
                        SNode::Body { value, prev } => self.head = Head::Some { value, prev },
                    }
                }

                Some(value)
            }
        }
    }

    pub fn peek(&self) -> Option<T> {
        match &self.head {
            Head::None => None,
            Head::One { value } | Head::Some { value, prev: _ } => Some(value.clone()),
        }
    }
}
