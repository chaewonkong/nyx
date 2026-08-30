use nyx::{lexer::Lexer, token::TokenKind};

fn main() {
    let src = "x = 10 + 20\n";
    let mut lx = Lexer::new(src);
    loop {
        let t = lx.next_token();
        println!("{:?}", t);
        if t.kind == TokenKind::Eof {
            break;
        }
    }
}
