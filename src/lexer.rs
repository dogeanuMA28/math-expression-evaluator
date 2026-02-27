use crate::tokens::Token;

fn get_symbol(c : char) -> Token {
	match c {
		'+' => return Token::Plus,
		'-' => return Token::Minus,
		'*' => return Token::Multiply,
		'/' => return Token::Division,
		'(' => return Token::LParen,
		')' => return Token::RParen,
		_   => unreachable!(),
	};
}

pub fn tokenize(expr: &str) -> Result<Vec<Token>, String> {
	if expr.is_empty() {
		return Err("Empty expression".to_string());
	}

	let mut tokens = Vec::new();
	let mut buf = String::new();

	for (i, c) in expr.chars().enumerate() {
		match c {
			// skip white space
			' ' => continue,
			// if it a number try to build it
			'1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {
				buf.push(c);

				if let Some(last) = tokens.last() {
					if matches!(last, Token::RParen) {
						return Err(format!("Not a valid number(coaie nu mai pune paranteze in numar)"));
					}
				}
			}
			// if it is an operation
			'(' => {
				if !buf.is_empty() {
					return Err(format!("LParen must be preceded by an operand!"));
				}

				let symb = get_symbol(c);

				tokens.push(symb);
			}
			'+' | '-' | '*' | '/' | ')' => {
				if tokens.is_empty() {
					if buf.is_empty() {
						return Err(format!("{}", i));
					}
					// parse the number
					let num = buf.parse().unwrap();
					buf.clear();
					tokens.push(Token::Number(num));
				} else {
					if let Some(last) = tokens.last() {
						if !matches!(last, Token::RParen) {
							if buf.is_empty() {
								return Err(format!("Expression does not end in a number! {}", i));
							}
							// parse the number
							let num = buf.parse().unwrap();
							buf.clear();
							tokens.push(Token::Number(num));
						}
					}
				}
				// get the symbol
				let symb = get_symbol(c);

				tokens.push(symb);
			}
			_ => return Err("Not a valid expression(unexpected character)!".to_string()),
		}
	}

	// do the number check again(it must end in a number)
	if let Some(last) = tokens.last() {
		if !matches!(last, Token::RParen) {
			if buf.is_empty() {
				return Err("Expression does not end in a number!".to_string());
			}
			// parse the number
			let num = buf.parse().unwrap();
			buf.clear();
			tokens.push(Token::Number(num));
		}
	}

	Ok(tokens)
}