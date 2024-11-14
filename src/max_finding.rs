use rocket::http::RawStr;
use serde::{Serialize, Deserialize};

use crate::helper;

fn instructions() -> helper::LongPage {
	helper::LongPage {
		title: "MAX-FINDING".to_string(),
		links:  calculus::links(5),
		instructions: "In the url bar after <tt>'https://basic-calculus.herokuapp.com/max-finding</tt> type the following:<p align=center>&sol;&lt;point at which to start search for a maximum&gt;&sol;&lt;function of <i>x</I>&gt;</tt></p>Note that this will not necessarily find the local maximum which is <i>closest</i> to the input point.".to_string(),
		note: format!("{}{}", helper::NOTE1, helper::NOTE2).to_string(),
		example: "To find a local maximum of the function sin <i>x</i> + <i>x</i>/2 while starting the search at <i>x</i> = 1, type <tt>/1/sin(x)+xd2</tt> after the current url address.  The coordinates for this result should be <tt>(2.094..., 1.913...)</tt>.  If you want to find a local m<i>in</I>imum, simply multiply your function by -1.".to_string(),
		algorithm: "simple bisection (and quadratic interpolation?)".to_string(),
		json: "Type '/json' in the url bar immediately after 'max-finding' if you would like the result in this format rather than html.  A successful response will contain six properties. 'xi' is the location where the search starts, 'x' is where the search ends, 'f' is the function value there, 'bracket_steps' is the number of steps required to find numbers on either side of (ie, to 'bracket') the maximum, and 'max_steps' is the subsequent number of steps required for the algorithm to find this maximum to within the absolute accuracy specified in the last property: 'epsilon'. An unsuccessful response will have one property: 'message' (a string reporting the error).".to_string(),
	}
}

pub fn page() -> String {helper::format(instructions())}

#[derive(Serialize, Deserialize, Debug)]
pub struct Results {
	pub xi: f64,
	pub x: f64,
	pub f: f64,
	pub bracket_steps: i32,
	pub max_steps: i32,
	pub epsilon: f64,
}

pub fn raw (xi_str: &RawStr, input_str: &RawStr) -> Result<Results, String> {
	let max_steps_max = 50;
	let epsilon = (10_f64).powf(-5.);
	let bracket_steps_max = 30;
	let xi = match helper::parse_expression(xi_str.to_string()) {
	  	Ok(xi) => xi,
	  	Err(message) => return Err(message),
	};
	let mut x1 = xi;
	// arbitrary
	let mut step = 0.1;
	// First, bracket the root.
	let mut x0 = x1 - step / 2.;
	let mut x2 = x1 + step / 2.;
	let mut f0 = match helper::function1(input_str.to_string(), x0) {
		Ok(f0) => f0,
		Err(message) => return Err(message),
	};
	let mut f1 = match helper::function1(input_str.to_string(), x1) {
		Ok(f1) => f1,
		Err(message) => return Err(message),
	};
	let mut f2 = match helper::function1(input_str.to_string(), x2) {
		Ok(f2) => f2,
		Err(message) => return Err(message),
	};
	let mut bracket_steps = 0;
	while f1 < f0 || f1 < f2 {
		// golden ratio
		step *= 1.6;
		if f2 > f0 {
			x0 = x1;
			f0 = f1;
			x1 = x2;
			f1 = f2;
			x2 += step;
			f2 = match helper::function1(input_str.to_string(), x2) {
				Ok(f2) => f2,
				Err(message) => return Err(message),
			};
		} else {
			x2 = x1;
			f2 = f1;
			x1 = x0;
			f1 = f0;
			x0 -= step;
			f0 = match helper::function1(input_str.to_string(), x0) {
				Ok(f0) => f0,
				Err(message) => return Err(message),
			};
		}
		bracket_steps += 1;
		if bracket_steps > bracket_steps_max {
			return Err(format!("Unable to bracket a max after {} steps.", bracket_steps_max));
		}
	}
	let mut max_steps = 0;
	// Following two vars will be sequential estimates using parabolic interpolation.
	let mut x_old = -f64::INFINITY;
	let mut x_new = f64::INFINITY;
	while (x_old - x_new).abs() > epsilon {
		if max_steps > max_steps_max {
			return Err(format!("Unable to locate a bracketed max within {} steps.", max_steps_max));
		}
		// Bisect the segment for which the outer function value is smallest.
		let x = (x1 + if f0 > f2 { x2 } else { x0 }) / 2.;
		let f = match helper::function1(input_str.to_string(), x) {
			Ok(f) => f,
			Err(message) => return Err(message),
		};
		if x < x1 {
			if f < f1 {
				x0 = x;
				f0 = f;
			} else {
				x2 = x1;
				f2 = f1;
				x1 = x;
				f1 = f;
			}
		} else {
			if f < f1 {
				x2 = x;
				f2 = f;
			} else {
				x0 = x1;
				f0 = f1;
				x1 = x;
				f1 = f;
			}
		}
		x_old = x_new;
		// parabolic interpolation
		let num = (x1 - x0) * (x1 - x0) * (f1 - f2) - (x1 - x2) * (x1 - x2) * (f1 - f0);
		let den = (x1 - x0) * (f1 - f2) - (x1 - x2) * (f1 - f0);
		x_new = x1 - num / den / 2.;
		max_steps += 1;
	}
	let f = match helper::function1(input_str.to_string(), x_new) {
		Ok(f) => f,
		Err(message) => return Err(message),
	};

	Ok(Results {
		xi,
		x: x_new,
		f,
		bracket_steps,
		max_steps,
		epsilon,
	})
}
