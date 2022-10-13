use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// dimensions of the board
   #[arg(short, long, default_value_t = 3)]
   dim: usize,

   /// Number of players
   #[arg(short, long, default_value_t = 2)]
   players: usize,
}

fn main() {
    let args = Args::parse();
    oandx::play_game(args.dim, args.players);
}
