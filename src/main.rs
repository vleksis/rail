use rail::lexer::{lexer::Lexer, token::TokenKind};

fn main() {
    let args: Vec<String> = std::env::args().into_iter().collect();
    let file = std::path::PathBuf::from(&args[1]);
    let file = std::fs::read_to_string(file).unwrap();
    let mut lexer = Lexer::new(&file);
    while let Ok(tok) = lexer.scan_token() {
        println!("{tok:?}");
        if tok.get_kind() == TokenKind::Eof {
            break;
        }
    }
}
