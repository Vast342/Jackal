use spear::{ChessBoard, Move, Piece, Side};

use super::NetworkLayer;

#[allow(non_upper_case_globals)]
pub static PolicyNetwork: PolicyNetwork = unsafe {
    std::mem::transmute(*include_bytes!(
        "../../../resources/networks/policy_001.network"
    ))
};

#[repr(C)]
struct PolicySubNetwork {
    l0: NetworkLayer<768, 16>,
}

impl PolicySubNetwork {
    pub fn forward(&self, inputs: &Vec<usize>) -> Vec<f32> {
        let mut l0_out = *self.l0.biases();

        for &weight_index in inputs {
            for (i, weight) in l0_out
                .values_mut()
                .iter_mut()
                .zip(&self.l0.weights()[weight_index].vals)
            {
                *i += *weight;
            }
        }

        l0_out.values().to_vec()
    }
}

#[repr(C)]
pub struct PolicyNetwork {
    subnets: [PolicySubNetwork; 128],
}

impl PolicyNetwork {
    pub fn forward(&self, inputs: &Vec<usize>, mv: Move, vertical_flip: u8) -> f32 {
        let from_index = (mv.get_from_square().get_raw() ^ vertical_flip) as usize;
        let to_index = (mv.get_to_square().get_raw() ^ vertical_flip) as usize;

        let from = self.subnets[from_index].forward(inputs);
        let to = self.subnets[to_index + 64].forward(inputs);

        dot(from, to)
    }

    pub fn map_policy_inputs<F: FnMut(usize), const STM_WHITE: bool, const NSTM_WHITE: bool>(
        board: &ChessBoard,
        mut method: F,
    ) {
        let flip = board.side_to_move() == Side::BLACK;

        for piece in Piece::PAWN.get_raw()..=Piece::KING.get_raw() {
            let piece_index = 64 * (piece - Piece::PAWN.get_raw()) as usize;

            let mut stm_bitboard =
                board.get_piece_mask_for_side::<STM_WHITE>(Piece::from_raw(piece));
            let mut nstm_bitboard =
                board.get_piece_mask_for_side::<NSTM_WHITE>(Piece::from_raw(piece));

            if flip {
                stm_bitboard = stm_bitboard.flip();
                nstm_bitboard = nstm_bitboard.flip();
            }

            stm_bitboard.map(|square| method(piece_index + (square.get_raw() as usize)));

            nstm_bitboard.map(|square| method(384 + piece_index + (square.get_raw() as usize)));
        }
    }
}

fn dot(a: Vec<f32>, b: Vec<f32>) -> f32 {
    let mut res = 0.0;

    for (i, j) in a.iter().zip(b) {
        res += relu(*i) * relu(j);
    }

    res
}

fn relu(x: f32) -> f32 {
    x.max(0.0)
}
