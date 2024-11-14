use rocket::http::RawStr;
use serde::{Serialize, Deserialize};

use crate::helper;

fn instructions() -> helper::LongPage {
	helper::LongPage {
		title: "DIFFERENTIATION".to_string(),
		links: calculus::links(2),
		instructions: "In the url bar after <tt>https://basic-calculus.herokuapp.com/differentiation</tt> type the following:<p align=center><tt>&sol;&lt;value of <i>x</i> at which to calculate function and derivatives&gt;&sol;&lt;function of <i>x</I>&gt;</tt></p>".to_string(),
		note: format!("{}{}{}", helper::NOTE1, "", helper::NOTE2),
		example: "To differentiate the function 2<i>x</i> + 3/(<i>x</i><sup>4</sup> + 5) at <i>x</i> = 1, type <tt>/1/2x+3d(x**4+5)</tt> after the current url address. The results for the values of the function and of its first three derivatives should be <tt>2.5, 1.66..., -0.55..., and 1.11...</tt>".to_string(),
		algorithm: "finite differences for small values of &Delta;<i>x</i>, excluding any reference to the particular point itself in the case of a removable singularity".to_string(),
		json: "Type '/json' in the url bar immediately after 'differentiation' if you would like the result in this format rather than html.  A successful response will contain three properties: 'x' (a float), 'nonsingular' (a boolean reflecting whether or not the function has a removable singularity), and 'derivs' (a 4-element array of floats whose values represent the function value and first through third derivatives, respectively).  An unsuccessful response will have one property: 'message' (a string reporting the error).".to_string(),
	}
}

pub fn page() -> String {helper::format(instructions())}

#[derive(Serialize, Deserialize, Debug)]
pub struct Results {
	pub x: f64,
	pub nonsingular: bool,
	pub derivs: Vec<f64>,
}

pub fn raw (x_str: &RawStr, input_str: &RawStr) -> Result<Results, String> {
	let x = match helper::parse_expression(x_str.to_string()) {
	  Ok(x) => x,
	  Err(message) => return Err(message),
	};
	let f = helper::function1(input_str.to_string(), x);
	let dx = 0.001;
	let steps = vec![2., 1., -1., -2.];
	let mut fs = vec![];
	for step in steps {
	  fs.push(match helper::function1(input_str.to_string(), x + step * dx) {
		Ok(f) => f,
		Err(message) => return Err(message),
	  });
	}
	let mut f0 = 0.;
	// I prob need to implement better testing for this.
	let nonsingular = f.is_ok();
	if nonsingular {
	  f0 = f.unwrap();
	}
	let derivs = vec![
	  // How to use values at discrete points to calculate function and derivative values
	  // For every other case, allowance needs to be made for a removable singularity.
	  if nonsingular {f0} else {(fs[1] + fs[2]) / 2.},
	  (fs[1] - fs[2]) / 2. / dx,
	  if nonsingular {(fs[1] - 2. * f0 + fs[2]) / dx / dx} else {(fs[0] - fs[1] - fs[2] + fs[3]) / 3. / dx / dx},
	  (fs[0] - fs[3] - 2. * fs[1] + 2. * fs[2]) / 2. / dx / dx / dx,
	];
	Ok(Results {
		x: x,
		nonsingular: nonsingular,
		derivs: derivs,
	})
}
