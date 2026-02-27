use crate::tokens::Token;

pub fn is_valid_expresion(tokens: Vec<Token>) -> bool {
	if tokens.is_empty() {
		return false;
	}

	let mut was_op = true;
	let mut paren_count = 0;

	for tok in tokens {
		match tok {
			Token::Number(_) => {
				if !was_op {
					return false;
				}

				was_op = false;
			}
			Token::Plus | Token::Minus | Token::Multiply | Token::Division => {
				if was_op {
					return false;
				}
				was_op = true;
			}
			Token::LParen => {
				if !was_op {
					return false;
				}
				paren_count += 1;
			}
			Token::RParen => {
				if was_op {
					return false;
				}

				paren_count -= 1;
				if paren_count < 0 {
					return false;
				}
			}
		}
	}

	return paren_count == 0 && !was_op;
}
