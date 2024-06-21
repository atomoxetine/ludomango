// 0..=15 -> initial cells
// 16..=67 -> common cells
// 68..=91 -> finish cells

// 16, 29, 42, 55 -> first common cells for each player
// 16, 24, 29, 37, 42, 50, 55, 64 -> safe cells
// 40, 53, 66, 27 -> common_to_finish cells
// 68, 74, 80, 86 -> first finish cells for each player

use std::fmt::Display;

const fn is_safe_cell(cell: u8) -> bool {
    match cell {
        16 | 24 | 29 | 37 | 42 | 50 | 55 | 63 => true,
        _ => false,
    }
}

const fn is_initial_cell(cell: u8) -> bool {
    cell <= 15
}

const fn is_finish_cell(cell: u8, player: u8) -> bool {
    let bg = 68 + 6 * player;
    let end = bg + 5;

    bg <= cell && cell <= end
}

const fn is_common_to_finish_cell(cell: u8, player: u8) -> bool {
    cell == match player {
        0 => 66,
        1 => 27,
        2 => 40,
        3 => 53,
        _ => panic!("invalid player id"),
    }
}

const fn next_cell(cell: u8, player: u8) -> Option<u8> {
    if is_initial_cell(cell) {
        return Some(16 + 13 * player);
    }

    if is_finish_cell(cell, player) {
        return if is_finish_cell(cell + 1, player) {
            Some(cell + 1)
        } else {
            None
        };
    }

    if is_common_to_finish_cell(cell, player) {
        return Some(68 + 6 * player);
    }

    Some(cell + 1)
}

fn advance(initial_cell: u8, player: u8, amount: u8) -> Option<u8> {
    (0..amount).try_fold(initial_cell, |cell, _| next_cell(cell, player))
}

#[derive(Debug)]
pub struct GameState {
    pieces: [u8; 16],
}

impl Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            (0..4).fold(String::new(), |acc, i| {
                format!(
                    "{acc}\nplayer {i}: {}",
                    self.pieces[(i * 4)..((i * 4) + 4)]
                        .iter()
                        .fold(String::new(), |acc, p| format!("{acc} {p}"))
                )
            })
        )
    }
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            pieces: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        }
    }

    fn piece_cell(&self, piece: u8) -> u8 {
        self.pieces[piece as usize]
    }

    fn pieces_in(&self, cell: u8) -> Vec<u8> {
        self.pieces
            .iter()
            .copied()
            .enumerate()
            .filter(|p| p.1 == cell)
            .map(|x| x.0 as u8)
            .collect()
    }

    fn player_pieces_in(&self, cell: u8, player: u8) -> Vec<u8> {
        self.pieces[(player as usize)..((player + 4) as usize)]
            .iter()
            .copied()
            .enumerate()
            .filter(|p| p.1 == cell)
            .map(|x| x.0 as u8)
            .collect()
    }

    fn move_to_initial(&mut self, piece: u8) {
        let player = piece / 4;
        (player..(player + 4)).try_for_each(|cell| {
            if self.player_pieces_in(cell, player).is_empty() {
                Some(())
            } else {
                self.pieces[piece as usize] = cell;
                None
            }
        });
    }

    pub fn move_piece(&mut self, piece: u8, amount: u8) -> Option<()> {
        let player = piece / 4;
        let cell = self.piece_cell(piece);
        let new_cell = advance(cell, player, amount)?;

        if !is_safe_cell(new_cell) {
            self.pieces_in(new_cell)
                .iter()
                .filter(|p| !(player..(player + 4)).contains(*p))
                .for_each(|p| self.move_to_initial(*p));
        }

        self.pieces[piece as usize] = new_cell;

        Some(())
    }
}
