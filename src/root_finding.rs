use rocket::http::RawStr;
use serde::{Serialize, Deserialize};

use crate::helper;

fn instructions() -> helper::LongPage {
	helper::LongPage {
		title: "ROOT-FINDING".to_string(),
		links:  calculus::links(4),
		instructions: "In the url bar after <tt>'https://basic-calculus.herokuapp.com/root-finding</tt> type the following:<p align=center>&sol;&lt;point at which to start search for a root&gt;&sol;&lt;function of <i>x</I>&gt;</tt></p>Note that this will not necessarily find the root which is <i>closest</i> to the input point.".to_string(),
		note: format!("{}{}", helper::NOTE1, helper::NOTE2).to_string(),
		example: "To find a root of the function 2<i>x</i> - 3/(<i>x</i><sup>4</sup> + 5) while starting the search at <i>x</i> = 1, type <tt>/1/2x-3d(x**4+5)</tt> after the current url address.  The result for this should be <tt>0.2995...</tt>".to_string(),
		algorithm: "alternating steps of inverse quadratic interpolation and simple bisection".to_string(),
		json: "Type '/json' in the url bar immediately after 'root-finding' if you would like the result in this format rather than html.  A successful response will contain five properties. 'xi' is the location where the search starts, 'x' is the root that is eventually found, 'bracket_steps' is the number of steps required to find numbers on either side of (ie, to 'bracket') the root, and 'root_steps' is the subsequent number of steps required for the algorithm to find this root to within the absolute accuracy specified in the last property: 'epsilon'. An unsuccessful response will have one property: 'message' (a string reporting the error).".to_string(),
	}
}

pub fn page() -> String {helper::format(instructions())}

#[derive(Serialize, Deserialize, Debug)]
pub struct Results {
	pub xi: f64,
	pub x: f64,
	pub bracket_steps: i32,
	pub root_steps: i32,
	pub epsilon: f64,
}

pub fn raw (xi_str: &RawStr, input_str: &RawStr) -> Result<Results, String> {
	let epsilon = (10_f64).powf(-12.);
	let bracket_steps_max = 30;
	let xi = match helper::parse_expression(xi_str.to_string()) {
	  	Ok(x0) => x0,
	  	Err(message) => return Err(message),
	};
	// arbitrary
	let mut step = 0.1;
	// First, bracket the root.
	let mut x0 = xi - step / 2.;
	let mut x2 = xi + step / 2.;
	let mut f0 = match helper::function1(input_str.to_string(), x0) {
		Ok(f0) => f0,
		Err(message) => return Err(message),
	};
	let mut f2 = match helper::function1(input_str.to_string(), x2) {
		Ok(f2) => f2,
		Err(message) => return Err(message),
	};
	let mut bracket_steps = 0;
	while f0 * f2 > 0. {
		// golden mean is optimal for this
		step *= 1.6;
		if f0.abs() < f2.abs() {
			x0 -= step;
			f0 = match helper::function1(input_str.to_string(), x0) {
				Ok(f0) => f0,
				Err(message) => return Err(message),
			};
		} else {
			x2 += step;
			f2 = match helper::function1(input_str.to_string(), x2) {
				Ok(f2) => f2,
				Err(message) => return Err(message),
			};
		}
		bracket_steps += 1;
		if bracket_steps > bracket_steps_max {
			return Err(format!("Unable to bracket a root after {} steps.", bracket_steps_max));
		}
	}
	// Second, find a root that has been bracketed.
	let root_steps_max = 20;
	let mut root_steps = 0;
	// Utilize a third point, to allow inverse-quadratic interpolation.
	let mut x1 = (x0 + x2) / 2.;
	let mut f1 = match helper::function1(input_str.to_string(), x1) {
		Ok(f1) => f1,
		Err(message) => return Err(message),
	};
	let mut bisect = true;
	while f0.abs() > epsilon && f1.abs() > epsilon && f2.abs() > epsilon && (x2 - x1) * (x1 - x0) > epsilon * epsilon {
		bisect = !bisect;
		if root_steps > root_steps_max {
			return Err(format!("Unable to locate a bracketed root within {} steps.", root_steps_max));
		}
		// Alternate between bisection and inverse-quadratic interpolation to get the safety of the former and speed of the latter.
		if bisect {
			if f0 * f1 > 0. {
				let xc = (x1 + x2) / 2.;
				let fc = match helper::function1(input_str.to_string(), xc) {
					Ok(fc) => fc,
					Err(message) => return Err(message),
				};
				if fc * f2 > 0. {
					f2 = fc;
					x2 = xc;
				} else {
					f1 = fc;
					x1 = xc;
				}
			} else {
				let xc = (x1 + x0) / 2.;
				let fc = match helper::function1(input_str.to_string(), xc) {
					Ok(fc) => fc,
					Err(message) => return Err(message),
				};
				if fc * f0 > 0. {
					f0 = fc;
					x0 = xc;
				} else {
					f1 = fc;
					x1 = xc;
				}
			}
		} else {
			// inverse-quadratic interpolation (See wikipedia.)
			let xc = x0 * f1 * f2 / (f0 - f1) / (f0 - f2) +
			         x1 * f2 * f0 / (f1 - f0) / (f1 - f2) +
					 x2 * f0 * f1 / (f2 - f0) / (f2 - f1);
			// if interpolation results are outside brackets, skip to bisection iteration
			if xc < x0 || xc > x2 {
				continue;
			}
			let fc = match helper::function1(input_str.to_string(), xc) {
				Ok(fc) => fc,
				Err(message) => return Err(message),
			};
			if fc * f1 > 0. {
				if xc < x1 {
					x2 = x1;
					f2 = f1;
				} else {
					x0 = x1;
					f0 = f1;
				}
				x1 = xc;
				f1 = fc;
			} else {
				if fc * f0 > 0. {
					x0 = xc;
					f0 = fc;
				} else {
					x2 = xc;
					f2 = fc;
				}
			}
		}
		if f1 == 0. {
			break;
		}
		root_steps += 1;
	}
	if f0 * f1 <= 0. {
		x1 = if f0.abs() < f1.abs() {x0} else {x1};
	} else {
		x1 = if f2.abs() < f1.abs() {x2} else {x1};
	}
	Ok(Results {
		xi,
		x: x1,
		bracket_steps,
		root_steps,
		epsilon,
	})
}
