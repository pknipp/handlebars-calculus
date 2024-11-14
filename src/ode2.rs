use rocket::http::RawStr;
use serde::{Serialize, Deserialize};

use crate::helper;

fn instructions() -> helper::LongPage {
	helper::LongPage {
		title: "2ND-ORDER DIFFERENTIAL EQUATIONS".to_string(),
		links:  calculus::links(7),
		instructions: "This page solves a differential equation of the form <i>d</I><sup>2</sup><i>x/dt</i><sup>2</sup> = function of <I>x</I>, of <I>dx/dt</I> (= '<i>v</I>'), and of 'time' <I>t</I>, with a specified 'initial condition', ie values of <I>x</I> and of <i>v</I> when the 'time' <i>t</i> = 0. In the url bar after <tt>'https://basic-calculus.herokuapp.com/ode2</tt> type the following:<p align=center>&sol;&lt;initial value of <i>x</I>&gt;&sol;&lt;initial value of <i>v</I> v&gt;&sol;&lt;final value of <i>t</I>&gt;&sol;&lt;number of time-steps&gt;&sol;&lt;function of <i>x</I>, <i>v</I>, and <i>t</I>&gt;</tt></p>".to_string(),
		note: format!("{}{}", helper::NOTE1, helper::NOTE2).to_string(),
		example: "To solve the equation d<sup>2</sup>/dt<sup>2</sup> = -2x - v + 3t with the initial conditions that x(0) = 0 and dx/dt = v(0) = 1 over the range 0 < t < 4 using 10 time-steps, type <tt>/0/1/4/10/-2x-v+3t</tt> after /ode2 in the url above.  In this case the final values for x and dx/dt should be 5.31... and 1.57..., respectively.".to_string(),
		algorithm: "4th-order Runge-Kutta method".to_string(),
		json: "Type '/json' in the url bar immediately after 'ode2' if you would like the result in this format rather than html.  All data are returned.".to_string(),
	}
}

pub fn page() -> String {helper::format(instructions())}

#[derive(Serialize, Deserialize, Debug)]
pub struct Results {
	pub xi: f64,
	pub vi: f64,
	pub tf: f64,
	pub nt: i32,
	pub xs: Vec<f64>,
	pub vs: Vec<f64>,
}

pub fn raw (xi_str: &RawStr, vi_str: &RawStr, tf_str: &RawStr, nt_str: &RawStr, input_str: &RawStr) -> Result<Results, String> {
	let xi = match helper::parse_expression(xi_str.to_string()) {
	  	Ok(x0) => x0,
	  	Err(message) => return Err(message),
	};
	let vi = match helper::parse_expression(vi_str.to_string()) {
		Ok(v0) => v0,
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
				return Err("Number of timesteps must be positive.".to_string());
			}
			nt as i32
		},
		Err(message) => return Err(message),
  	};
	let mut xs = vec![xi];
	let mut vs = vec![vi];
	let dt = tf / (nt as f64);
	for i in 0..nt {
		let t = (i as f64) * tf / (nt as f64);
		let x = xs[i as usize];
		let v = vs[i as usize];
		let v1 = v;
		let a1 = match helper::function3(input_str.to_string(), x, t, v) {
			Ok(a) => a,
			Err(message) => return Err(message),
		};
		let v2 = v + a1 * dt / 2.;
		let a2 = match helper::function3(input_str.to_string(), x + v * dt / 2., t + dt / 2., v2) {
			Ok(a) => a,
			Err(message) => return Err(message),
		};
		let v3 = v + a2 * dt / 2.;
		let a3 = match helper::function3(input_str.to_string(), x + v2 * dt / 2., t + dt / 2., v3) {
			Ok(a) => a,
			Err(message) => return Err(message),
		};
		let v4 = v + a3 * dt;
		let a4 = match helper::function3(input_str.to_string(), x + v3 * dt, t + dt, v4) {
			Ok(a) => a,
			Err(message) => return Err(message),
		};
		xs.push(x + ((v1 + v4) + 2. * (v2 + v3)) * dt / 6.);
		vs.push(v + ((a1 + a4) + 2. * (a2 + a3)) * dt / 6.);
	}
	return Ok(Results {xi, vi, tf, nt, xs, vs});
}
