#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Variable {
    pub name: String,
    pub offset: u64,
    pub next: Option<Box<Variable>>,
}

impl Variable {
    pub fn new(name: String, offset: u64, next: Option<Box<Variable>>) -> Self {
        Self { name, offset, next }
    }

    pub fn push(&mut self, name: String, offset: u64) {
        let old_next = self.next.take();
        self.next = Some(Box::new(Variable::new(name, offset, old_next)));
    }

    pub fn find(&self, name: &str) -> Option<u64> {
        let mut current = self;
        while let Some(next) = &current.next {
            if next.name == name {
                return Some(next.offset);
            }
            current = next;
        }
        None
    }
}
