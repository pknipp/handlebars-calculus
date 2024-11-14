use rocket::http::RawStr;
use serde::{Serialize, Deserialize};

use crate::helper;

fn instructions() -> helper::LongPage {
	helper::LongPage {
		title: "INTEGRATION".to_string(),
		links: calculus::links(3),
		instructions: "In the url bar after <tt>'https://basic-calculus.herokuapp.com/integration</tt> type the following:<p align=center>&sol;&lt;lower limit of integration&gt;&sol;&lt;upper limit of integration&gt;&sol;&lt;function of <i>x</I>&gt;</tt></p>Neither singularities (integrable or otherwise) nor infinite ranges of integration are allowed.".to_string(),
		note: format!("{}{}{}", helper::NOTE1, "", helper::NOTE2).to_string(),
		example: "To integrate the function 2<i>x</i> + 3/(<i>x</i><sup>4</sup> + 5) from <i>x</i> = 1 to 6, type <tt>/1/6/2x+3d(x**4+5)</tt> after the current url address.  The result for this should be <tt>35.41...</tt>".to_string(),
		algorithm: "composite Simpson's rule and Aitken extrapolation".to_string(),
		json: "Type '/json' in the url bar immediately after 'integration' if you would like the result in this format rather than html.  A successful response will contain five properties. 'xi' and 'xf' are the lower and upper limits of integration, 'integral' is the value of the definite integral, and 'subdivisions' is the number of equally sized intervals into which the range of integration needed to be subdivided in order to achieve the absolute accuracy specified in the last property: 'epsilon'. An unsuccessful response will have one property: 'message' (a string reporting the error)".to_string(),
	}
}

pub fn page() -> String {helper::format(instructions())}

#[derive(Serialize, Deserialize, Debug)]
pub struct Results {
	pub xi: f64,
	pub xf: f64,
	pub integral: f64,
	pub subdivisions: i32,
	pub epsilon: f64,
}

pub fn raw(xi_str: &RawStr, xf_str: &RawStr, input_str: &RawStr) -> Result<Results, String> {
	let epsilon = (10_f64).powf(-12.);
	struct Pt {
		x: f64,
		f: f64,
		wt: f64,
	}
	let mut pts = vec![];
	for x_str in &[xi_str, xf_str] {
		let x = match helper::parse_expression(x_str.to_string()) {
			Ok(x) => x,
			Err(message) => return Err(message),
		};
		let f = match helper::function1(input_str.to_string(), x) {
			Ok(f) => f,
			Err(message) => return Err(message),
		};
		pts.push(Pt{x, f, wt: 0.5}); // non-0th pt will only reside in vector for an instant
	}
	let ptf = match pts.pop() { // final point will be handled separately, going forward
	  	Some(ptf) => ptf,
	  	None => return Err("Missing integration endpoint".to_string()),
	};
	let mut integral = f64::INFINITY;
	// variables needed to implement Aitken's algo to accelerate a geometric sequence
	let mut aitkens = f64::INFINITY;
	let mut aitkens_new = f64::INFINITY;
	let mut dx = ptf.x - pts[0].x; // interval for Simpson's rule
	let mut number = 1;
	while !aitkens.is_finite() || !aitkens_new.is_finite() || (aitkens_new - aitkens).abs() > epsilon {
		number *= 2;
		let mut integral_new = ptf.f * ptf.wt;
		let mut new_pts = vec![];
		dx /= 2.; // start preparing next set of integration points
		for mut pt in pts {
			integral_new += pt.f * pt.wt;
			pt.wt = 1.; // wt for most points is 1 except for their first appearance
			let x = pt.x + dx; // x-coord of next point
			let f = match helper::function1(input_str.to_string(), x) {
			  	Ok(f) => f,
			  	Err(message) => return Err(format!("Cannot evaluate function at x: {}{}", pt.x, message)),
			};
			new_pts.append(&mut vec![pt, Pt{x, f, wt: 2.}]);
		}
		integral_new *= 4. * dx / 3.; // overall factor, for extended Simpson's rule
		pts = new_pts; // Overwrite pts vector, which was moved during iteration
		pts[0].wt = 0.5; // wt of 0th and last points is always 0.5 (ie, never 1.)
		aitkens = aitkens_new;
		aitkens_new = integral_new;
		if integral.is_finite() {
			// Aitken's correction, because integral's accuracy is O(dx^4)
			aitkens_new += (integral_new - integral ) / (16. - 1.);
		}
		integral = integral_new;
	}
	Ok(Results{
		integral: aitkens_new,
		xi: pts[0].x,
	  	xf: ptf.x,
		subdivisions: number,
		epsilon: epsilon,
	})
}
