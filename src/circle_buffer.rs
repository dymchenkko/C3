use std::collections::VecDeque;

#[derive(Debug)]
pub struct CircleBuffer<T> {
    length: usize,
    data: Option<VecDeque<T>>
}

impl <T> CircleBuffer<T> {
    pub fn new (length: usize) -> Self {
        CircleBuffer {
            length,
            data: None
        }
    }

    pub fn add (&mut self, element: T) {
        match &mut self.data {
            Some(ref mut vector) => {
                if vector.len() == self.length {
                    let _ = vector.pop_front();
                }
                vector.push_back(element);
            },
            None => self.data = Some(vec![element].into()),
        }
    }
}