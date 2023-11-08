pub struct Node {
    //coordinates or something
}

impl Node {
    #[inline]
    pub fn new() -> Self {
        Node {}
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::new()
    }
}
