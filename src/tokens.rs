#[derive(Debug, Clone)]
pub enum Token {
	Number(f64),
	Plus,
	Multiply,
	Minus,
	Division,
	RParen,
	LParen,
}