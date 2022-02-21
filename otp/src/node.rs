use rand::prelude::*;
use rand_distr::Weibull;

#[derive(Debug, Clone, Copy)]
pub struct Node {
    pub node_id: usize,
    pub children: [isize; 2],
    pub value: u32,
}

impl Node {
    pub fn switch(node_id: usize) -> Self {
        // State in Lemonade paper at section 6.4
        // scale parameter = 10
        // shape parameter = 1
        let random: f64 = thread_rng().sample(Weibull::new(1000.5, 1.).unwrap());

        Self {
            node_id,
            children: [-1, -1],
            value: random as u32,
        }
    }
    pub fn memory(node_id: usize) -> Self {
        let random = thread_rng().gen::<u8>();
        Self {
            node_id,
            children: [-1, -1],
            value: random as u32,
        }
    }
}
