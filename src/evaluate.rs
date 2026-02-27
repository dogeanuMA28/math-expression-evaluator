use crate::tokens::Token;
use std::{collections::VecDeque};
struct Context {
	add_seq: Vec<f64>,
    mul_seq: Vec<f64>,
    op: char,
}

pub fn evaluate(tokens: Vec<Token>) -> f64 {
    let mut tokens: VecDeque<Token> = tokens.into();

	let mut paren_stack: Vec<Context> = Vec::new();
	let mut add_seq: Vec<f64> = Vec::new();
	let mut mul_seq: Vec<f64> = Vec::new();
	let mut last_op: char = ' ';
	while let Some(tok) = tokens.pop_front() {
		match tok {
			Token::Number(n) => {
				let symb = tokens.pop_front(); 
				match symb {
					Some(Token::Plus)     => { 
						match last_op {
							'+' => add_seq.push(n),
							'-' => add_seq.push(-n),
							'*' => mul_seq.push(n),
							'/' => mul_seq.push(1.0 / n),
							_   => add_seq.push(n),
						} 
						last_op = '+'; 
						if !mul_seq.is_empty() {
							let acc: f64 = mul_seq.iter().product();
							mul_seq.clear();
							add_seq.push(acc);
						}
					}
					Some(Token::Minus)    => { 
						match last_op {
							'+' => add_seq.push(n),
							'-' => add_seq.push(-n),
							'*' => mul_seq.push(n),
							'/' => mul_seq.push(1.0 / n),
							_   => add_seq.push(-n),
						} 
						last_op = '-'; 
						if !mul_seq.is_empty() {
							let acc: f64 = mul_seq.iter().product();
							mul_seq.clear();
							add_seq.push(acc);
						}
					}
					Some(Token::Multiply) => { 
						match last_op {
							'+' => mul_seq.push(n),
							'-' => mul_seq.push(-n),
							'*' => mul_seq.push(n),
							'/' => mul_seq.push(1.0 / n),
							_   => mul_seq.push(n),
						}  
						last_op = '*'; 
					}
					Some(Token::Division) => { 
						match last_op {
							'+' => mul_seq.push(n),
							'-' => mul_seq.push(-n),
							'*' => mul_seq.push(n),
							'/' => mul_seq.push(1.0 / n),
							_   => mul_seq.push(n),
						}   
						last_op = '/'; 
					}
					Some(Token::RParen)   => {
						match last_op {
							'+' => add_seq.push(n),
							'-' => add_seq.push(-n),
							'*' => mul_seq.push(n),
							'/' => mul_seq.push(1.0 / n),
							_   => return 87.0,
						} 
						let mut mul_acc: f64 = 0.0;
						if !mul_seq.is_empty() {
							mul_acc = mul_seq.iter().product();
							mul_seq.clear();
						}

						add_seq.push(mul_acc);
						let add_acc: f64 = add_seq.iter().sum();
						add_seq.clear();

						if let Some(Context { add_seq: old_add,
											  mul_seq: old_mul,
											  op: old_op }) = paren_stack.pop() {
								add_seq = old_add;
								mul_seq = old_mul;
								last_op = old_op;
							}

						match last_op {
							'+' => add_seq.push(add_acc),
							'-' => add_seq.push(-add_acc),
							'*' => mul_seq.push(add_acc),
							'/' => mul_seq.push(1.0 / add_acc),
							_   => {
								let next_op = tokens.pop_front();
								match next_op {
									Some(Token::Plus) => { add_seq.push(add_acc); last_op = '+'; }
									Some(Token::Minus) => { add_seq.push(add_acc); last_op = '-'; }
									Some(Token::Multiply) => { mul_seq.push(add_acc); last_op = '*';}
									Some(Token::Division) => { mul_seq.push(add_acc); last_op = '/';}
									_ => return 69.0,
								} 
							}
						}
					}
					_ 					  => {
						match last_op {
							'+' => add_seq.push(n),
							'-' => add_seq.push(-n),
							'*' => mul_seq.push(n),
							'/' => mul_seq.push(1.0 / n),
							_   => return 67.0,
						} 
					}
				}
			}
			Token::LParen => {
				paren_stack.push(Context{add_seq: add_seq,
										 mul_seq: mul_seq,
										 op: last_op,
										});
				add_seq = Vec::new();
				mul_seq = Vec::new();
				last_op = ' ';
			}
			Token::RParen => {
				let mut mul_acc: f64 = 0.0;
				if !mul_seq.is_empty() {
					mul_acc = mul_seq.iter().product();
					mul_seq.clear();
				}

				add_seq.push(mul_acc);
				let add_acc: f64 = add_seq.iter().sum();
				add_seq.clear();

				if let Some(Context { add_seq: old_add,
											  mul_seq: old_mul,
											  op: old_op }) = paren_stack.pop() {
					add_seq = old_add;
					mul_seq = old_mul;
					last_op = old_op;
				}

				match last_op {
					'+' => add_seq.push(add_acc),
					'-' => add_seq.push(-add_acc),
					'*' => mul_seq.push(add_acc),
					'/' => mul_seq.push(1.0 / add_acc),
					_   => {
						let next_op = tokens.pop_front();
						match next_op {
							Some(Token::Plus) => { add_seq.push(add_acc); last_op = '+'; }
							Some(Token::Minus) => { add_seq.push(add_acc); last_op = '-'; }
							Some(Token::Multiply) => { mul_seq.push(add_acc); last_op = '*';}
							Some(Token::Division) => { mul_seq.push(add_acc); last_op = '/';}
							_ => return 69.0,
						} 
					}
				}
			}
			_ => {println!("{:?}", tok); return 66.0;}
		}
	}

	if !mul_seq.is_empty() {
		let acc: f64 = mul_seq.iter().product();
		add_seq.push(acc);
	}

	return add_seq.iter().sum();
}