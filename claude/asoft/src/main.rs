use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug, Clone)]
enum Value {
    Number(f64),
    String(String),
}

#[derive(Debug, Clone)]
enum Token {
    Number(f64),
    String(String),
    Identifier(String),
    Keyword(String),
    Operator(String),
    Comma,
    Semicolon,
    LeftParen,
    RightParen,
    Equals,
}

struct Interpreter {
    lines: HashMap<usize, String>,
    variables: HashMap<String, Value>,
    pc: Option<usize>,
    return_stack: Vec<usize>,
    data: Vec<Value>,
    data_pointer: usize,
}

impl Interpreter {
    fn new() -> Self {
	Interpreter {
	    lines: HashMap::new(),
	    variables: HashMap::new(),
	    pc: None,
	    return_stack: Vec::new(),
	    data: Vec::new(),
	    data_pointer: 0,
	}
    }

    fn tokenize(&self, line: &str) -> Vec<Token> {
	let mut tokens = Vec::new();
	let mut chars = line.chars().peekable();

	while let Some(&ch) = chars.peek() {
	    if ch.is_whitespace() {
		chars.next();
		continue;
	    }

	    if ch.is_digit(10) || ch == '.' {
		let mut num_str = String::new();
		while let Some(&c) = chars.peek() {
		    if c.is_digit(10) || c == '.' {
			num_str.push(c);
			chars.next();
		    } else {
			break;
		    }
		}
		tokens.push(Token::Number(num_str.parse().unwrap_or(0.0)));
	    } else if ch == '"' {
		chars.next();
		let mut str_val = String::new();
		while let Some(c) = chars.next() {
		    if c == '"' {
			break;
		    }
		    str_val.push(c);
		}
		tokens.push(Token::String(str_val));
	    } else if ch.is_alphabetic() {
		let mut ident = String::new();
		while let Some(&c) = chars.peek() {
		    if c.is_alphanumeric() || c == '$' {
			ident.push(c);
			chars.next();
		    } else {
			break;
		    }
		}
		let upper = ident.to_uppercase();
		match upper.as_str() {
		    "PRINT" | "LET" | "IF" | "THEN" | "GOTO" | "GOSUB" | "RETURN" |
		    "FOR" | "TO" | "STEP" | "NEXT" | "INPUT" | "END" | "REM" |
		    "DIM" | "DATA" | "READ" | "RESTORE" | "AND" | "OR" | "NOT" => {
			tokens.push(Token::Keyword(upper));
		    }
		    _ => tokens.push(Token::Identifier(ident)),
		}
	    } else {
		match ch {
		    '=' => { tokens.push(Token::Equals); chars.next(); }
		    ',' => { tokens.push(Token::Comma); chars.next(); }
		    ';' => { tokens.push(Token::Semicolon); chars.next(); }
		    '(' => { tokens.push(Token::LeftParen); chars.next(); }
		    ')' => { tokens.push(Token::RightParen); chars.next(); }
		    '+' | '-' | '*' | '/' | '^' => {
			tokens.push(Token::Operator(ch.to_string()));
			chars.next();
		    }
		    '<' | '>' => {
			let mut op = ch.to_string();
			chars.next();
			if let Some(&next_ch) = chars.peek() {
			    if next_ch == '=' || (ch == '<' && next_ch == '>') {
				op.push(next_ch);
				chars.next();
			    }
			}
			tokens.push(Token::Operator(op));
		    }
		    _ => { chars.next(); }
		}
	    }
	}
	tokens
    }

    fn eval_expr(&mut self, tokens: &[Token], pos: &mut usize) -> Result<Value, String> {
	self.eval_or(tokens, pos)
    }

    fn eval_or(&mut self, tokens: &[Token], pos: &mut usize) -> Result<Value, String> {
	let mut left = self.eval_and(tokens, pos)?;

	while *pos < tokens.len() {
	    if let Token::Keyword(kw) = &tokens[*pos] {
		if kw == "OR" {
		    *pos += 1;
		    let right = self.eval_and(tokens, pos)?;
		    let left_num = self.value_to_num(&left)?;
		    let right_num = self.value_to_num(&right)?;
		    left = Value::Number(if left_num != 0.0 || right_num != 0.0 { 1.0 } else { 0.0 });
		} else {
		    break;
		}
	    } else {
		break;
	    }
	}
	Ok(left)
    }

    fn eval_and(&mut self, tokens: &[Token], pos: &mut usize) -> Result<Value, String> {
	let mut left = self.eval_comparison(tokens, pos)?;

	while *pos < tokens.len() {
	    if let Token::Keyword(kw) = &tokens[*pos] {
		if kw == "AND" {
		    *pos += 1;
		    let right = self.eval_comparison(tokens, pos)?;
		    let left_num = self.value_to_num(&left)?;
		    let right_num = self.value_to_num(&right)?;
		    left = Value::Number(if left_num != 0.0 && right_num != 0.0 { 1.0 } else { 0.0 });
		} else {
		    break;
		}
	    } else {
		break;
	    }
	}
	Ok(left)
    }

    fn eval_comparison(&mut self, tokens: &[Token], pos: &mut usize) -> Result<Value, String> {
	let mut left = self.eval_add_sub(tokens, pos)?;

	while *pos < tokens.len() {
	    if let Token::Operator(op) = &tokens[*pos] {
		if matches!(op.as_str(), "<" | ">" | "<=" | ">=" | "<>" | "=") {
		    let op = op.clone();
		    *pos += 1;
		    let right = self.eval_add_sub(tokens, pos)?;
		    let result = match (left, right) {
			(Value::Number(l), Value::Number(r)) => {
			    let cmp = match op.as_str() {
				"<" => l < r,
				">" => l > r,
				"<=" => l <= r,
				">=" => l >= r,
				"<>" => l != r,
				"=" => l == r,
				_ => false,
			    };
			    Value::Number(if cmp { 1.0 } else { 0.0 })
			}
			(Value::String(l), Value::String(r)) => {
			    let cmp = match op.as_str() {
				"<" => l < r,
				">" => l > r,
				"=" => l == r,
				"<>" => l != r,
				_ => false,
			    };
			    Value::Number(if cmp { 1.0 } else { 0.0 })
			}
			_ => return Err("Type mismatch".to_string()),
		    };
		    left = result;
		} else {
		    break;
		}
	    } else {
		break;
	    }
	}
	Ok(left)
    }

    fn eval_add_sub(&mut self, tokens: &[Token], pos: &mut usize) -> Result<Value, String> {
	let mut left = self.eval_mul_div(tokens, pos)?;

	while *pos < tokens.len() {
	    if let Token::Operator(op) = &tokens[*pos] {
		if op == "+" || op == "-" {
		    let op = op.clone();
		    *pos += 1;
		    let right = self.eval_mul_div(tokens, pos)?;
		    left = match (left, right) {
			(Value::Number(l), Value::Number(r)) => {
			    Value::Number(if op == "+" { l + r } else { l - r })
			}
			_ => return Err("Type mismatch".to_string()),
		    };
		} else {
		    break;
		}
	    } else {
		break;
	    }
	}
	Ok(left)
    }

    fn eval_mul_div(&mut self, tokens: &[Token], pos: &mut usize) -> Result<Value, String> {
	let mut left = self.eval_power(tokens, pos)?;

	while *pos < tokens.len() {
	    if let Token::Operator(op) = &tokens[*pos] {
		if op == "*" || op == "/" {
		    let op = op.clone();
		    *pos += 1;
		    let right = self.eval_power(tokens, pos)?;
		    left = match (left, right) {
			(Value::Number(l), Value::Number(r)) => {
			    Value::Number(if op == "*" { l * r } else { l / r })
			}
			_ => return Err("Type mismatch".to_string()),
		    };
		} else {
		    break;
		}
	    } else {
		break;
	    }
	}
	Ok(left)
    }

    fn eval_power(&mut self, tokens: &[Token], pos: &mut usize) -> Result<Value, String> {
	let mut left = self.eval_unary(tokens, pos)?;

	if *pos < tokens.len() {
	    if let Token::Operator(op) = &tokens[*pos] {
		if op == "^" {
		    *pos += 1;
		    let right = self.eval_power(tokens, pos)?;
		    left = match (left, right) {
			(Value::Number(l), Value::Number(r)) => Value::Number(l.powf(r)),
			_ => return Err("Type mismatch".to_string()),
		    };
		}
	    }
	}
	Ok(left)
    }

    fn eval_unary(&mut self, tokens: &[Token], pos: &mut usize) -> Result<Value, String> {
	if *pos < tokens.len() {
	    if let Token::Operator(op) = &tokens[*pos] {
		if op == "-" {
		    *pos += 1;
		    let val = self.eval_primary(tokens, pos)?;
		    return match val {
			Value::Number(n) => Ok(Value::Number(-n)),
			_ => Err("Type mismatch".to_string()),
		    };
		}
	    } else if let Token::Keyword(kw) = &tokens[*pos] {
		if kw == "NOT" {
		    *pos += 1;
		    let val = self.eval_primary(tokens, pos)?;
		    let num = self.value_to_num(&val)?;
		    return Ok(Value::Number(if num == 0.0 { 1.0 } else { 0.0 }));
		}
	    }
	}
	self.eval_primary(tokens, pos)
    }

    fn eval_primary(&mut self, tokens: &[Token], pos: &mut usize) -> Result<Value, String> {
	if *pos >= tokens.len() {
	    return Err("Unexpected end of expression".to_string());
	}

	match &tokens[*pos] {
	    Token::Number(n) => {
		let val = *n;
		*pos += 1;
		Ok(Value::Number(val))
	    }
	    Token::String(s) => {
		let val = s.clone();
		*pos += 1;
		Ok(Value::String(val))
	    }
	    Token::Identifier(name) => {
		let name = name.clone();
		*pos += 1;
		self.variables.get(&name)
		    .cloned()
		    .ok_or_else(|| format!("Undefined variable: {}", name))
	    }
	    Token::LeftParen => {
		*pos += 1;
		let val = self.eval_expr(tokens, pos)?;
		if *pos < tokens.len() && matches!(tokens[*pos], Token::RightParen) {
		    *pos += 1;
		}
		Ok(val)
	    }
	    _ => Err(format!("Unexpected token: {:?}", tokens[*pos])),
	}
    }

    fn value_to_num(&self, val: &Value) -> Result<f64, String> {
	match val {
	    Value::Number(n) => Ok(*n),
	    Value::String(_) => Err("Type mismatch: expected number".to_string()),
	}
    }

    fn execute_line(&mut self, line: &str) -> Result<Option<usize>, String> {
	let tokens = self.tokenize(line);
	if tokens.is_empty() {
	    return Ok(None);
	}

	let mut pos = 0;
	match &tokens[pos] {
	    Token::Keyword(kw) => {
		match kw.as_str() {
		    "PRINT" => {
			pos += 1;
			let mut first = true;
			let mut newline = true;
			while pos < tokens.len() {
			    if matches!(tokens[pos], Token::Semicolon) {
				pos += 1;
				newline = false;
				continue;
			    }
			    if matches!(tokens[pos], Token::Comma) {
				pos += 1;
				print!("\t");
				first = false;
				newline = true;
				continue;
			    }
			    if !first {
				print!(" ");
			    }
			    let val = self.eval_expr(&tokens, &mut pos)?;
			    match val {
				Value::Number(n) => print!("{}", n),
				Value::String(s) => print!("{}", s),
			    }
			    first = false;
			    newline = true;
			}
			if newline {
			    println!();
			}
		    }
		    "LET" => {
			pos += 1;
			if let Token::Identifier(name) = &tokens[pos] {
			    let name = name.clone();
			    pos += 1;
			    if pos < tokens.len() && matches!(tokens[pos], Token::Equals) {
				pos += 1;
				let val = self.eval_expr(&tokens, &mut pos)?;
				self.variables.insert(name, val);
			    }
			}
		    }
		    "IF" => {
			pos += 1;
			let condition = self.eval_expr(&tokens, &mut pos)?;
			let cond_true = self.value_to_num(&condition)? != 0.0;

			if pos < tokens.len() {
			    if let Token::Keyword(kw) = &tokens[pos] {
				if kw == "THEN" {
				    pos += 1;
				}
			    }
			}

			if cond_true && pos < tokens.len() {
			    if let Token::Number(line_num) = tokens[pos] {
				return Ok(Some(line_num as usize));
			    } else {
				let rest: Vec<Token> = tokens[pos..].to_vec();
				let rest_line = self.reconstruct_line(&rest);
				return self.execute_line(&rest_line);
			    }
			}
		    }
		    "GOTO" => {
			pos += 1;
			if let Token::Number(line_num) = tokens[pos] {
			    return Ok(Some(line_num as usize));
			}
		    }
		    "GOSUB" => {
			pos += 1;
			if let Token::Number(line_num) = tokens[pos] {
			    if let Some(current_pc) = self.pc {
				self.return_stack.push(current_pc);
			    }
			    return Ok(Some(line_num as usize));
			}
		    }
		    "RETURN" => {
			if let Some(ret_line) = self.return_stack.pop() {
			    return Ok(Some(ret_line));
			}
		    }
		    "INPUT" => {
			pos += 1;
			while pos < tokens.len() {
			    if let Token::Identifier(name) = &tokens[pos] {
				let name = name.clone();
				print!("? ");
				io::stdout().flush().unwrap();
				let mut input = String::new();
				io::stdin().read_line(&mut input).unwrap();
				let input = input.trim();

				if name.ends_with('$') {
				    self.variables.insert(name, Value::String(input.to_string()));
				} else {
				    let num: f64 = input.parse().unwrap_or(0.0);
				    self.variables.insert(name, Value::Number(num));
				}
				pos += 1;
				if pos < tokens.len() && matches!(tokens[pos], Token::Comma) {
				    pos += 1;
				}
			    } else {
				break;
			    }
			}
		    }
		    "END" => {
			return Err("END".to_string());
		    }
		    "REM" => {
			// Comment - ignore rest of line
		    }
		    _ => {}
		}
	    }
	    Token::Identifier(name) => {
		// Implicit LET
		let name = name.clone();
		pos += 1;
		if pos < tokens.len() && matches!(tokens[pos], Token::Equals) {
		    pos += 1;
		    let val = self.eval_expr(&tokens, &mut pos)?;
		    self.variables.insert(name, val);
		}
	    }
	    _ => {}
	}
	Ok(None)
    }

    fn reconstruct_line(&self, tokens: &[Token]) -> String {
	tokens.iter().map(|t| match t {
	    Token::Number(n) => n.to_string(),
	    Token::String(s) => format!("\"{}\"", s),
	    Token::Identifier(id) => id.clone(),
	    Token::Keyword(kw) => kw.clone(),
	    Token::Operator(op) => op.clone(),
	    Token::Comma => ",".to_string(),
	    Token::Semicolon => ";".to_string(),
	    Token::LeftParen => "(".to_string(),
	    Token::RightParen => ")".to_string(),
	    Token::Equals => "=".to_string(),
	}).collect::<Vec<_>>().join(" ")
    }

    fn run(&mut self) -> Result<(), String> {
	let mut line_nums: Vec<usize> = self.lines.keys().cloned().collect();
	line_nums.sort();

	let mut i = 0;
	while i < line_nums.len() {
	    let line_num = line_nums[i];
	    self.pc = Some(line_num);

	    if let Some(line) = self.lines.get(&line_num).cloned() {
		match self.execute_line(&line) {
		    Ok(Some(goto_line)) => {
			if let Some(pos) = line_nums.iter().position(|&l| l == goto_line) {
			    i = pos;
			    continue;
			} else {
			    return Err(format!("Line {} not found", goto_line));
			}
		    }
		    Ok(None) => {}
		    Err(msg) if msg == "END" => break,
		    Err(e) => return Err(e),
		}
	    }
	    i += 1;
	}
	Ok(())
    }

    fn repl(&mut self) {
	println!("Applesoft BASIC Interpreter");
	println!("Type LIST to see program, RUN to execute, NEW to clear");

	loop {
	    print!("] ");
	    io::stdout().flush().unwrap();

	    let mut input = String::new();
	    io::stdin().read_line(&mut input).unwrap();
	    let input = input.trim();

	    if input.is_empty() {
		continue;
	    }

	    let parts: Vec<&str> = input.splitn(2, ' ').collect();

	    if let Ok(line_num) = parts[0].parse::<usize>() {
		if parts.len() > 1 {
		    self.lines.insert(line_num, parts[1].to_string());
		} else {
		    self.lines.remove(&line_num);
		}
	    } else {
		match input.to_uppercase().as_str() {
		    "RUN" => {
			self.pc = None;
			self.return_stack.clear();
			if let Err(e) = self.run() {
			    if e != "END" {
				println!("Error: {}", e);
			    }
			}
		    }
		    "LIST" => {
			let mut line_nums: Vec<usize> = self.lines.keys().cloned().collect();
			line_nums.sort();
			for line_num in line_nums {
			    if let Some(line) = self.lines.get(&line_num) {
				println!("{} {}", line_num, line);
			    }
			}
		    }
		    "NEW" => {
			self.lines.clear();
			self.variables.clear();
		    }
		    "EXIT" | "QUIT" => break,
		    _ => {
			if let Err(e) = self.execute_line(input) {
			    println!("Error: {}", e);
			}
		    }
		}
	    }
	}
    }
}

fn main() {
    let mut interpreter = Interpreter::new();
    interpreter.repl();
}
