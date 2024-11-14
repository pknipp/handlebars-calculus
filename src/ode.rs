use rocket::http::RawStr;
use serde::{Serialize, Deserialize};

use crate::helper;

fn instructions() -> helper::LongPage {
	helper::LongPage {
		title: "1ST-ORDER DIFFERENTIAL EQUATIONS".to_string(),
		links:  calculus::links(6),
		instructions: "This page solves a differential equation of the form <i>dx/dt</I> = function of <I>x</I> and <I>t</I>, with a specified 'initial condition', ie a value of <I>x</I> when the 'time' <i>t</i> = 0.  In the url bar after <tt>'https://basic-calculus.herokuapp.com/ode</tt> type the following:<p align=center>&sol;&lt;initial value of <i>x</I>&gt;&sol;&lt;final value of <i>t</I>&gt;&sol;&lt;number of time-steps&gt;&sol;&lt;function of <i>x</I> and <i>t</I>&gt;</tt></p>".to_string(),
		note: format!("{}{}", helper::NOTE1, helper::NOTE2).to_string(),
		example: "To solve the equation dx/dt = 2x - t - 2 from t = 0 to t = 2 using 10 time steps and the initial condition that x(0) = 1, type <tt>/1/2/10/2x-t-2</tt> after /ode in the url above.  The final result should be that x(2) = -11.39..".to_string(),
		algorithm: "4th-order Runge-Kutta method".to_string(),
		json: "Type '/json' in the url bar immediately after 'ode' if you would like the result in this format rather than html.  All of the data are returned.".to_string(),
	}
}

pub fn page() -> String {helper::format(instructions())}

#[derive(Serialize, Deserialize, Debug)]
pub struct Results {
	pub xi: f64,
	pub tf: f64,
	pub nt: i32,
	pub xs: Vec<f64>,
}

pub fn raw (xi_str: &RawStr, tf_str: &RawStr, nt_str: &RawStr, input_str: &RawStr) -> Result<Results, String> {
	let xi = match helper::parse_expression(xi_str.to_string()) {
	  	Ok(x0) => x0,
	  	Err(message) => return Err(message),
	};
	let tf = match helper::parse_expression(tf_str.to_string()) {
		Ok(tf) => tf,
		Err(message) => return Err(message),
  	};
	let nt = match helper::parse_expression(nt_str.to_string()) {
		Ok(nt) => {
			if nt.round() != nt {
				return Err(format!("{} is not an integer.", nt));
			} else if nt <= 0. {
				return Err("number of timesteps must be positive.".to_string());
			}
			nt as i32
		},
		Err(message) => return Err(message),
  	};
	let mut xs = vec![xi];
	let dt = tf / (nt as f64);
	for i in 0..nt {
		let t = (i as f64) * tf / (nt as f64);
		let x = xs[i as usize];
		let v1 = match helper::function2(input_str.to_string(), x, t) {
			Ok(v) => v,
			Err(message) => return Err(message),
		};
		let v2 = match helper::function2(input_str.to_string(), x + v1 * dt / 2., t + dt / 2.) {
			Ok(v) => v,
			Err(message) => return Err(message),
		};
		let v3 = match helper::function2(input_str.to_string(), x + v2 * dt / 2., t + dt / 2.) {
			Ok(v) => v,
			Err(message) => return Err(message),
		};
		let v4 = match helper::function2(input_str.to_string(), x + v3 * dt, t + dt) {
			Ok(v) => v,
			Err(message) => return Err(message),
		};
		xs.push(x + ((v1 + v4) + 2. * (v2 + v3)) * dt / 6.);
	}
	return Ok(Results {xi, tf, nt, xs});
}
