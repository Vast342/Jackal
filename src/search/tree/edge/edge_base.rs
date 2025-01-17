use std::sync::atomic::{AtomicI16, AtomicU16, AtomicU32, Ordering};

use crate::search::{eval_score::AtomicScore, NodeIndex, Score};
use spear::Move;

pub struct Edge {
    node_index: AtomicU32,
    mv: AtomicU16,
    policy: AtomicI16,
    visits: AtomicU32,
    score: AtomicScore,
}

impl Clone for Edge {
    fn clone(&self) -> Self {
        Self {
            node_index: AtomicU32::new(self.node_index().get_raw()),
            mv: AtomicU16::new(self.mv().get_raw()),
            policy: AtomicI16::new(self.policy.load(Ordering::Relaxed)),
            visits: AtomicU32::new(self.visits()),
            score: self.score.clone(),
        }
    }
}

impl Default for Edge {
    fn default() -> Self {
        Self::new(NodeIndex::NULL, Move::NULL, 1.0)
    }
}

impl Edge {
    pub fn new(node_index: NodeIndex, mv: Move, policy: f32) -> Self {
        Self {
            node_index: AtomicU32::new(node_index.get_raw()),
            mv: AtomicU16::new(mv.get_raw()),
            policy: AtomicI16::new((policy * f32::from(i16::MAX)) as i16),
            visits: AtomicU32::new(0),
            score: AtomicScore::default(),
        }
    }

    #[inline]
    pub fn clear(&self) {
        self.replace(NodeIndex::NULL, Move::NULL, 1.0);
    }

    #[inline]
    pub fn replace(&self, node_index: NodeIndex, mv: Move, policy: f32) {
        self.set_node_index(node_index);
        self.mv.store(mv.get_raw(), Ordering::Relaxed);
        self.update_policy(policy);
        self.visits.store(0, Ordering::Relaxed);
        self.score.store(Score::default());
    }

    #[inline]
    pub fn node_index(&self) -> NodeIndex {
        NodeIndex::from_raw(self.node_index.load(Ordering::Relaxed))
    }

    #[inline]
    pub fn set_node_index(&self, index: NodeIndex) {
        self.node_index.store(index.get_raw(), Ordering::Relaxed);
    }

    #[inline]
    pub fn mv(&self) -> Move {
        Move::from_raw(self.mv.load(Ordering::Relaxed))
    }

    #[inline]
    pub fn policy(&self) -> f32 {
        f32::from(self.policy.load(Ordering::Relaxed)) / f32::from(i16::MAX)
    }

    #[inline]
    pub fn visits(&self) -> u32 {
        self.visits.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn score(&self) -> Score {
        self.score.load()
    }

    #[inline]
    pub fn add_score(&self, score: Score) {
        let score = f64::from(score);
        let previous_visits = self.visits.fetch_add(1, Ordering::Relaxed) as f64;
        let new_score =
            (f64::from(self.score()) * previous_visits + score) / (previous_visits + 1.0);
        self.score.store(Score::from(new_score))
    }

    #[inline]
    pub fn update_policy(&self, new_policy: f32) {
        self.policy
            .store((new_policy * f32::from(i16::MAX)) as i16, Ordering::Relaxed)
    }
}
