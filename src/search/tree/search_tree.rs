use std::{ops::{Index, IndexMut}, sync::atomic::{AtomicI32, Ordering}};

use spear::Move;

use super::{node::GameState, Edge, Node};

pub struct SearchTree {
    values: Vec<Node>,
    root_edge: Edge,
    last_index: AtomicI32
}

impl SearchTree {
    pub fn new() -> Self {
        let tree = Self {
            values: Vec::new(),
            root_edge: Edge::new(0, Move::NULL, 0.0),
            last_index: AtomicI32::new(0)
        };
        tree.init_root();
        tree
    }

    pub fn clear(&mut self) {
        self.values.clear();
        self.root_edge = Edge::new(0, Move::NULL, 0.0);
        self.init_root();
    }

    fn init_root(&self) {
        let root_index = self.spawn_node(GameState::Unresolved);
        self.root_edge.set_index(root_index);
    }

    pub fn root_index(&self) -> i32 {
        self.root_edge.index()
    }

    pub fn spawn_node(&self, state: GameState) -> i32 {
        let new_node_index = self.last_index.load(Ordering::Relaxed);
        self[new_node_index].replace(state);
        self.last_index.fetch_add(1, Ordering::Relaxed);
        new_node_index as i32
    }
}

impl Index<i32> for SearchTree {
    type Output = Node;

    fn index(&self, index: i32) -> &Self::Output {
        &self.values[index as usize]
    }
}

impl IndexMut<i32> for SearchTree {
    fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        &mut self.values[index as usize]
    }
}