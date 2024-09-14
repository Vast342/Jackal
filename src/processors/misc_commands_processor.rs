use crate::spear::{Move, Perft, Side, FEN};

use crate::{search::SearchEngine, utils::{clear_terminal_screen, heat_color}};

pub struct MiscCommandsProcessor;
impl MiscCommandsProcessor {
    //Handles commands that are not UCI related and returns true, if command was found, so it can skip uci commands
    pub fn execute(command: &str, args: &[String], search_engine: &SearchEngine) -> bool {
        match command {
            "exit" | "quit" | "q" => std::process::exit(0),
            "clean" | "clear" | "cls" | "cln" => clear_terminal_screen(),
            "perft" => {
                Self::perft::<false>(args, &search_engine.current_position().board().get_fen())
            }
            "bulk" => {
                Self::perft::<true>(args, &search_engine.current_position().board().get_fen())
            }
            "draw" | "d" => search_engine.current_position().board().draw_board(),
            "moves" => Self::moves(search_engine),
            _ => return false,
        }

        true
    }

    //Performs move generator performance test
    fn perft<const BULK: bool>(args: &[String], current_fen: &FEN) {
        //Obtain test depth from command arguments
        let depth = if args.len() == 0 {
            5
        } else {
            let parse = args[0].parse::<u8>();
            if parse.is_err() {
                return;
            }
            parse.unwrap()
        };

        Perft::perft::<BULK, true, true>(current_fen, depth);
    }

    //Prints all legal moves together with thier policy
    fn moves(search_engine: &SearchEngine) {
        println!("All legal moves");
        let mut moves: Vec<(Move, f32)> = Vec::new();
        if search_engine.current_position().board().side_to_move() == Side::WHITE {
            search_engine.current_position().board().map_moves::<_, true, false>(|mv| {
                moves.push((mv, 1.0))
            })
        } else {
            search_engine.current_position().board().map_moves::<_, false, true>(|mv| {
                moves.push((mv, 1.0))
            })
        }

        let moves_length = moves.len() as f32;
        for (_, policy) in &mut moves {
            *policy /= moves_length
        }

        for (index, &(mv, policy)) in moves.iter().enumerate() {
            let arrow = if index == moves.len() - 1 {
                "└─>"
            } else {
                "├─>"
            };
            println!("{} {:<4} {} - {}", arrow, format!("{}.", index + 1), mv, heat_color(format!("{:.2}%", policy * 100.0).as_str(), policy, 0.0, 1.0))
        }
    }
}
