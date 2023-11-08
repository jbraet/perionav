#[derive(Debug)]
pub struct Edge {
    node_a: i32,
    node_b: i32,
    distance: f64,

    //These properties should become mode specific
    speed: f64,
    is_forward: bool, //from node a to node b
    is_backward: bool,
}

impl Edge {
    #[inline]
    pub fn new(
        node_a: i32,
        node_b: i32,
        distance: f64,
        is_forward: bool,
        is_backward: bool,
    ) -> Self {
        Edge {
            node_a,
            node_b,
            distance,
            is_forward,
            is_backward,
            speed: 1.0,
        }
    }

    pub fn has_node(&self, node: i32) -> bool {
        self.node_a == node || self.node_b == node
    }

    //if base_node is neither of the edge nodes then the result is either one of the edge nodes
    pub fn get_adj_node(&self, base_node: i32) -> i32 {
        if base_node == self.node_a {
            self.node_b
        } else {
            self.node_a
        }
    }

    pub fn get_is_forward(&self, base_node: i32) -> bool {
        if base_node == self.node_a {
            self.is_forward
        } else {
            self.is_backward
        }
    }

    pub fn get_is_backward(&self, base_node: i32) -> bool {
        if base_node == self.node_a {
            self.is_backward
        } else {
            self.is_forward
        }
    }

    pub fn get_speed(&self) -> f64 {
        self.speed
    }

    pub fn get_distance(&self) -> f64 {
        self.distance
    }

    pub fn apply_nodes<F>(&self, mut f: F)
    where
        F: FnMut(i32, i32),
    {
        if self.is_forward {
            f(self.node_a, self.node_b)
        }
        if self.is_backward {
            f(self.node_b, self.node_a)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        let edge = Edge::new(0, 1, 0.0, true, true);
        assert_eq!(edge.get_adj_node(0), 1);
        assert_eq!(edge.get_adj_node(1), 0);
    }
}
