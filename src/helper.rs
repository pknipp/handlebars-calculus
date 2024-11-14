use std::f64::consts::PI;

pub const INSTRUCTIONS: &str = "WELCOME TO MY CALCULUS APP";

const FUNCTION: &str = "The function may be any algebraically legal combination of the variable letter(s), numbers, parentheses, and/or binary operations +, -, *, ** (encouraged) or ^ (discouraged), PI and/or the most common unary functions: <tt>abs, acos, acosh, acot, acoth, acsc, acsch, asec, asech, asin, asinh, atan, atanh, cbrt, ceil, cos, cot, csc, exp, exp2, exp_m1, floor, fract, ln, ln_1p, log10, log2, round, sec, signum, sin, sqrt, tan, and trunc</tt>.  (See <a href='https://doc.rust-lang.org/std/primitive.f64.html'>docs</a> for more information.) To represent division you must use either <tt>div, DIV, d, or D</tt> because the usual division symbol (<tt>/</tt>) has special meaning in a url.  Implied multiplication is allowed.  Spaces are allowed but discouraged.";

pub const NOTE1: &str = "The construction rules for the values of any variable in the url";
pub const NOTE2: &str = " are the same as those for the function except - of course - it cannot include the letter which represents the variable.";

pub struct LongPage {
	pub title: String,
	pub links: String,
	pub instructions: String,
	pub note: String,
	pub example: String,
	pub algorithm: String,
	pub json: String,
}

pub fn format(long_page: LongPage) -> String {
	format!(
		"<p align=center>{}</p>{}<br>{} {}<br>{}<br><b>example:</b>
		{}<br><b>algorithms:</b> {}<br><b>json:</b> {}",
		long_page.title,
		long_page.links,
		long_page.instructions,
		FUNCTION,
		long_page.note,
		long_page.example,
		long_page.algorithm,
		long_page.json,
	)
}

// precedence of binary operations
fn prec(op: &char) -> i32 {
	match op {
		'+'|'-' => 0,
		'*'|'/' => 1,
		'^' => 2,
		_ => unreachable!(),
	}
}

pub fn preparse (expression: &mut String, x: f64) {
	*expression = expression.to_lowercase();
	// following are replacements of url encoding of ^ and space, respectively.
	*expression = str::replace(&expression, "%5", &"^".to_string());
	*expression = str::replace(&expression, "%20", &"".to_string());
	// temporary swap-out of exp-spelling prevents confusion when inserting x value.
	*expression = str::replace(&expression, "exp", &"EXP".to_string());
	*expression = str::replace(&expression, "x", &format!("({})", x));
	*expression = str::replace(&expression, "EXP", &"exp".to_string());
}

pub fn function1(mut expression: String, x: f64) -> Result<f64, String> {
	preparse(&mut expression, x);
	parse_expression(expression.to_string())
}

pub fn function2(mut expression: String, x: f64, t: f64) -> Result<f64, String> {
	preparse(&mut expression, x);
	expression = str::replace(&expression, "t", &format!("({})", t).to_string());
	parse_expression(expression.to_string())
}

pub fn function3(mut expression: String, x: f64, t: f64, v: f64) -> Result<f64, String> {
	preparse(&mut expression, x);
	expression = str::replace(&expression, "t", &format!("({})", t).to_string());
	expression = str::replace(&expression, "v", &format!("({})", v).to_string());
	parse_expression(expression.to_string())
}

fn find_size (expression: &str) -> Result<usize, String> {
	let mut n_paren = 1; // leading (open)paren has been found, in calling function
	for (n_expression, char) in expression.chars().enumerate() {
		n_paren += if char == '(' {1} else if char == ')' {-1} else {0};
		if n_paren == 0 {
			// Closing parenthesis has been found.
			return Ok(n_expression);
		}
	}
	Err(format!("Error: no closing parenthesis was found for this string: {}", expression))
}

fn get_value(expression: &mut String) -> Result<f64, String> {
	if expression.is_empty() {
		return Err("Error: your expression truncates prematurely.".to_string());
	}
	let mut value = 0.;
	if expression.starts_with('(') {
		// remove leading parenthesis
		expression.remove(0);
		let n_expression = match find_size(expression) {
			Ok(n_expression) => n_expression,
			Err(message) => return Err(message),
		};
		// recursive call to evaluate what is in parentheses
		value = match parse_expression((&expression[..n_expression]).to_string()) {
			Err(message) => return Err(message),
			Ok(value) => value,
		};
		// From expression remove trailing parenthesis and characters preceding it.
		*expression = expression.split_off(n_expression + 1);
	// A letter here triggers that we are starting a unary function name (or E-notation?)
	} else if expression.chars().next().unwrap().is_alphabetic() {
		let mut method = String::from("");
		let mut found_paren = false;
		while !expression.is_empty() {
			let char = expression.remove(0);
			if char == '(' {
				found_paren = true;
				break;
			} else {
				method += &String::from(char);
			}
		}
		if !found_paren {
			return Err(format!("Error: the unary function {} does not seem to have an argument.", method));
		}
		let n_expression = match find_size(expression) {
			Ok(n_expression) => n_expression,
			Err(message) => return Err(format!("Error: could not find length of argument string ({}) for function ({}): {}", expression, method, message)),
		};
		// recursive call, for argument of unary
		let arg = match parse_expression((expression)[..n_expression].to_string()) {
			Ok(arg) => arg,
			Err(message) => return Err(format!("Error: could not parse argument {}: {}", expression, message)),
		};
		value = match unary(&method, arg) {
			Ok(value) => value,
			Err(message) => return Err(message),
		};
		// Trim argument of unary from beginning of expression
		*expression = expression.split_off(n_expression + 1);
	} else {
		let mut found_value = false;
		let mut p = 1; // index which tracks progress thru expression
		let mut x = "";
		while expression.len() >= p {
			x = &expression[..p];
			if !(x == "." || x == "-" || x == "-.") { // It's premature to parse for a number.
				value = match x.parse() {
					Ok(value) => {
						found_value = true;
						value // This may get more digit(s) in next iteration(s).
					},
					Err(_) => break, // either found end of value or expression is unparsable
				};
			}
			p += 1;
		}
		if x.starts_with("-") && p == 2 && expression.len() > 1 { // examples of this edge case: -sin(x) or -(x+1)**2
			value = -1.;
			found_value = true;
		}
		if !found_value {
			return Err(format!("Error: cannot parse a number from the start of '{}'", expression));
		}
		*expression = expression.split_off(p - 1); //start of expression is no longer needed
	}
	Ok(value)
}

fn binary(x1: f64, op: &char, x2: f64) -> Result<f64, String> {
	let x = match op {
		'+' => x1 + x2,
		'-' => x1 - x2,
		'*' => x1 * x2,
		'/' => {
			if x2 == 0. {
				return Err(format!("Error: {}/0 signifies an attempt to divide by zero", x1));
			}
			x1 / x2
		},
		'^' => {
			if x2 <= 0. && x1 == 0. {
				return Err(format!("Error: {}^0 is ill-defined.", x1));
			}
			x1.powf(x2)
		},
		_ => unreachable!(),
	};
	Ok(x)
}

pub fn parse_expression(mut expression: String) -> Result<f64, String> {
	preparse(&mut expression, 0.);
	expression = str::replace(&expression, "pi", &format!("({})", PI)); // important constant
  	for stri in ["div", "DIV", "d", "D"] {
    	expression = str::replace(&expression, stri, "/"); // division operation is a special URL char
  	}
	expression = str::replace(&expression, "**", "^"); // in case user chooses ^ instead of **

	if !expression.is_empty() {
		// leading "+" may be trimmed thoughtlessly
		if expression.starts_with('+') {
			expression.remove(0);
		}
	}
	// Elements of these two vectors are interleaved: val/op/val/op.../op/val
	let mut vals = vec![];
	let mut ops = vec![];
	// trim & push leading number from expression
	vals.push(match get_value(&mut expression) {
		Err(message) => return Err(message),
		Ok(value) => value,
	});
	let op_string = "+-*/^";
	// loop thru the expression, while trimming & pushing operation/number pairs
	while !expression.is_empty() {
		let op = expression.chars().next().unwrap();
		// The following ternary includes an implied multiplication, if appropriate.
		ops.push(if op_string.contains(op) {expression.remove(0)} else {'*'});
		vals.push(match get_value(&mut expression) {
			Err(message) => return Err(message),
			Ok(value) => value,
		});
	}
	// loop thru "ops" vector, evaluating operations in order of their precedence
	while !ops.is_empty() {
		let mut index = 0;
		while ops.len() > index {
			if index < ops.len() - 1 && prec(&ops[index]) < prec(&ops[index + 1]) {
				// postpone this operation because of its lower prececence
				index += 1;
			} else {
				// perform this operation NOW, because of PEMDAS rule
				match binary(vals[index], &ops[index], vals[index + 1]) {
					Err(message) => return Err(message),
					Ok(result) => {
						// mutate vals & ops (including the shortening of both by one)
						vals[index] = result;
						ops.remove(index);
						vals.remove(index + 1);
					},
				};
				// Start another loop thru the expression, ISO high-precedence operations.
				index = 0;
			}
		}
	}
	Ok(vals[0]) // what remains after ops vector is emptied
}

fn is_nonzero(x: f64) -> Result<f64, String> {
	if x == 0. {Err("Error: divide by zero".to_string())} else {Ok(x)}
}

fn unary(method: &str, x: f64) -> Result<f64, String> {
	let negative = format!("is not defined for negative argument such as {}", x);
	let nonpositive = format!("is not defined for a nonpositive argument such as {}", x);
	match method {
		"abs" => Ok(x.abs()),
		"acos" => {
			if x<= 1. {
				Ok(x.acos())
			} else {
				return Err(format!("Error evaluating acos({}): your argument cannot exceed 1 in absolute value.", x))
			}
		},
		"acosh" => {
			if x >= 1. {
				Ok(x.acosh())
			} else {
				return Err(format!("Error evaluating acosh({}): your argument cannot be smaller than 1.", x))
			}
		},
		"acot" => {
			if x == 0. {
				Ok(PI/2.)
			} else if x > 0. {
				Ok((1./x).atan())
			} else {
				Ok(PI + (1./x).atan())
			}
		},
		"acoth" => {
			if x.abs() > 1. {
				Ok((1./x).atanh())
			} else {
				return Err(format!("Error evaluating acoth({}): argument's absolute value must exceed 1.", x))
			}
		},
		"acsc" => {
			if x.abs() >= 1. {
				Ok((1./x).asin())
			} else {
				return Err(format!("Error evaluating acsc({}): argument's absolute value may not be smaller than 1.", x))
			}
		},
		"acsch" => match is_nonzero(x) {
			Ok(x) => Ok((1./x).asinh()),
			Err(message) => return Err(format!("Error evaluating {}({}): {}", method, x, message)),
		},
		"asec" => {
			if x.abs() < 1. {
				return Err(format!("Error evaluating asec({}): argument's absoluate value may not be less than 1.", x))
			} else {
				Ok((1./x).acos())
			}
		},
		"asech" => {
			if x > 0. && x <= 1. {
				Ok((1./x).acosh())
			} else {
				return Err(format!("Error evaluating asech({}): argument must be between 0 (exclusive) and 1 (inclusive).", x))
			}
		},
		"asin" => {
			if x.abs() < 1. {
				return Err(format!("Error evaluating asin({}): argument's absolute value may not exceed 1.", x))
			} else {
				Ok(x.asin())
			}
		},
		"asinh" => Ok(x.asinh()),
		"atan" => Ok(x.atan()),
		"atanh" => {
			if x.abs() < 1. {
				Ok(x.atanh())
			} else {
				return Err(format!("Error evaluating atanh({}): argument's absolute value must be less than 1.", x))
			}
		},
		"cbrt" => Ok(x.cbrt()),
		"ceil" => Ok(x.ceil()),
		"cos" => Ok(x.cos()),
		"cot" => match is_nonzero(x) {
			Ok(x) => Ok(x.cos()/x.sin()),
			Err(message) => return Err(format!("Error evaluating {}({}): {}", method, x, message)),
		},
		"csc" => match is_nonzero(x) {
			Ok(x) => Ok(1./x.sin()),
			Err(message) => return Err(format!("Error evaluating {}({}): {}", method, x, message)),
		},
		"exp" => Ok(x.exp()),
		"exp2" => Ok(x.exp2()),
		"exp_m1" => Ok(x.exp_m1()),
		"floor" => Ok(x.floor()),
		"fract" => Ok(x.fract()),
		"ln" => {
			if x > 0. {
				Ok(x.ln())
			} else {
				Err(format!("Error: ln {}", nonpositive))
			}
		},
		"ln_1p" => {
			if x > -1. {
				Ok(x.ln_1p())
			} else {
				Err(format!("Error evaluating ln_1p({}): argument must exceed -1", x))
			}
		},
		"log10" => {
			if x > 0. {
				Ok(x.log10())
			} else {
				Err(format!("Error: log10 {}", nonpositive))
			}
		},
		"log2" => {
			if x > 0. {
				Ok(x.log2())
			} else {
				Err(format!("Error: log2 {}", nonpositive))
			}
		},
		"round" => Ok(x.round()),
		"sec" => Ok(1./x.cos()),
		"signum" => Ok(x.signum()),
		"sin" => Ok(x.sin()),
		"sqrt" => {
			if x < 0. {
				Err(format!("Error: sqrt {}", negative))
			} else {
				Ok(x.sqrt())
			}
		},
		"tan" => Ok(x.tan()),
		"trunc" => Ok(x.trunc()),
		_ => Err(format!("Error: no such function: {}", method)),
	}
}
