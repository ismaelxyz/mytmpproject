use std::io::{self, BufRead, Write};
use crate::game::{parse_coord, find_best_move, Board, Player};

pub fn play_game<R: BufRead, W: Write>(mut reader: R, mut writer: W) -> io::Result<()> {
    writeln!(writer, "Tres en raya — Juego contra la máquina")?;

    // Elegir símbolo
    let user_player: Player;
    loop {
        write!(writer, "Elige X u O: ")?;
        writer.flush()?;
        let mut input = String::new();
        if reader.read_line(&mut input)? == 0 {
            return Ok(());
        }
        let ch = input.trim().to_uppercase();
        if ch == "X" {
            user_player = Player::X;
            break;
        } else if ch == "O" {
            user_player = Player::O;
            break;
        } else {
            writeln!(writer, "Entrada inválida. Escribe X u O.")?;
        }
    }

    let ai_player = user_player.other();
    writeln!(writer, "Jugarás con {}. La máquina es {}.", user_player.to_char(), ai_player.to_char())?;

    let mut board = Board::new();
    let mut turn = Player::X; // X siempre comienza

    loop {
        board.print_to(&mut writer)?;

        if let Some(w) = board.winner() {
            writeln!(writer, "Gana {}!", w.to_char())?;
            break;
        }
        if board.is_full() {
            writeln!(writer, "Empate.")?;
            break;
        }

        if turn == user_player {
            // Turno del usuario
            loop {
                write!(writer, "Tu turno (fila:col -> 0:0 ... 2:2): ")?;
                writer.flush()?;
                let mut input = String::new();
                if reader.read_line(&mut input)? == 0 {
                    return Ok(());
                }
                if let Some((r, c)) = parse_coord(&input) {
                    if board.place(r, c, user_player) {
                        break;
                    } else {
                        writeln!(writer, "Casilla ocupada o coordenada inválida.")?;
                    }
                } else {
                    writeln!(writer, "Formato inválido. Usa e.g. 1:2")?;
                }
            }
        } else {
            // Turno de la IA
            writeln!(writer, "Turno de la máquina ({}). Pensando...", ai_player.to_char())?;
            let best = find_best_move(&board, ai_player);
            if let Some(mv) = best {
                let r = mv / 3;
                let c = mv % 3;
                board.place(r, c, ai_player);
                writeln!(writer, "La máquina juega {}:{}", r, c)?;
            } else {
                if let Some(&mv) = board.available_moves().first() {
                    let r = mv / 3;
                    let c = mv % 3;
                    board.place(r, c, ai_player);
                    writeln!(writer, "La máquina juega {}:{}", r, c)?;
                }
            }
        }

        turn = turn.other();
    }

    board.print_to(&mut writer)?;
    writeln!(writer, "Fin del juego.")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn play_game_user_wins() {
        // User chooses X and plays a quick winning sequence
        // Inputs: X, user moves: 0:0, 1:0, 2:0 -> creates a column win
        let input = b"X\n0:0\n1:1\n1:0\n0:1\n2:0\n"; // mixed to account AI moves
        let reader = Cursor::new(&input[..]);
        let mut out = Vec::new();
        play_game(reader, &mut out).unwrap();
        let s = String::from_utf8(out).unwrap();
        assert!(s.contains("Gana" ) || s.contains("Empate"));
    }

    #[test]
    fn invalid_choice_then_play() {
        let input = b"Z\nO\n0:0\n"; // invalid choice then O
        let reader = Cursor::new(&input[..]);
        let mut out = Vec::new();
        play_game(reader, &mut out).unwrap();
        let s = String::from_utf8(out).unwrap();
        assert!(s.contains("Entrada inválida") && s.contains("La máquina"));
    }
}
