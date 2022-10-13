mod board;
mod players;

use board::Board;
use board::HasWon;
use players::Players;

use rand::Rng;

type MoveType = String;

pub fn play_game(dim: usize, no_of_players:usize) {
    let mut board = Board::new(dim);
    let mut players = Players::new(no_of_players);

    rand_play(&mut board, &mut players);

    board.display();
    
    match board.has_won() {
        HasWon::Yes(p) => println!("{p} has won the game"),
        HasWon::No => println!("Game is a draw"),
    }
}

fn rand_play(board: &mut Board, players: &mut Players) {

    let mut v: Vec<(usize, usize)> = Vec::new();
    for r in 0..board.dim {
        for c in 0..board.dim {
            v.push((r,c));
        }
    }

    while v.len() > 0 {
        let pos = rand::thread_rng().gen_range(0..v.len());
        let p = players.get_player_and_next();
        board.add_move(v[pos].0, v[pos].1, &p);
        v.remove(pos);
    }
}