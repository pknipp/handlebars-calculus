#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use rocket::http::RawStr;
use rocket::response::content;

mod helper;

mod differentiation;
mod integration;
mod root_finding;
mod max_finding;
mod ode;
mod ode2;

extern crate calculus;
extern crate serde_json;

#[get("/")]
fn index() -> content::Html<String> {
  content::Html(calculus::general_page())
}

#[get("/differentiation")]
fn differentiation_page() -> content::Html<String> {
  content::Html(differentiation::page())
}

#[get("/integration")]
fn integration_page() -> content::Html<String> {
  content::Html(integration::page())
}

#[get("/root-finding")]
fn root_finding_page() -> content::Html<String> {
  content::Html(root_finding::page())
}

#[get("/max-finding")]
fn max_finding_page() -> content::Html<String> {
  content::Html(max_finding::page())
}

#[get("/ode")]
fn ode_page() -> content::Html<String> {
  content::Html(ode::page())
}

#[get("/ode2")]
fn ode2_page() -> content::Html<String> {
  content::Html(ode2::page())
}

#[get("/differentiation/json/<x_str>/<input_str>")]
fn differentiation_json(x_str: &RawStr, input_str: &RawStr) -> String {
  match differentiation::raw(x_str, input_str) {
    Ok(results) => serde_json::to_string(&results).unwrap(),
    Err(message) => format!("{{\"message\": {}}}", message),
  }
}

#[get("/integration/json/<xi_str>/<xf_str>/<input_str>")]
fn integration_json(xi_str: &RawStr, xf_str: &RawStr, input_str: &RawStr) -> String {
  match integration::raw(xi_str, xf_str, input_str) {
    Ok(results) => serde_json::to_string(&results).unwrap(),
    Err(message) => format!("{{\"message\": {}}}", message),
  }
}

#[get("/root-finding/json/<x_str>/<input_str>")]
fn root_finding_json(x_str: &RawStr, input_str: &RawStr) -> String {
  match root_finding::raw(x_str, input_str) {
    Ok(results) => serde_json::to_string(&results).unwrap(),
    Err(message) => format!("{{\"message\": {}}}", message),
  }
}

#[get("/max-finding/json/<x_str>/<input_str>")]
fn max_finding_json(x_str: &RawStr, input_str: &RawStr) -> String {
  match max_finding::raw(x_str, input_str) {
    Ok(results) => serde_json::to_string(&results).unwrap(),
    Err(message) => format!("{{\"message\": {}}}", message),
  }
}

#[get("/ode/json/<x_str>/<t_str>/<nt_str>/<input_str>")]
fn ode_json(
  x_str: &RawStr,
  t_str: &RawStr,
  nt_str: &RawStr,
  input_str: &RawStr
) -> String {
  match ode::raw(x_str, t_str, nt_str, input_str) {
    Ok(results) => serde_json::to_string(&results).unwrap(),
    Err(message) => format!("{{\"message\": {}}}", message),
  }
}

#[get("/ode2/json/<x_str>/<v_str>/<t_str>/<nt_str>/<input_str>")]
fn ode2_json(
  x_str: &RawStr,
  v_str: &RawStr,
  t_str: &RawStr,
  nt_str: &RawStr,
  input_str: &RawStr
) -> String {
  match ode2::raw(x_str, v_str, t_str, nt_str, input_str) {
    Ok(results) => serde_json::to_string(&results).unwrap(),
    Err(message) => format!("{{\"message\": {}}}", message),
  }
}

#[get("/differentiation/<x_str>/<input_str>")]
fn differentiation_html(x_str: &RawStr, input_str: &RawStr) -> content::Html<String> {
  let instructions = differentiation::page();
  let results = match differentiation::raw(x_str, input_str) {
    Ok(results) => results,
    Err(message) => return content::Html(format!("{}<br><br><b>result</b> for the function f(x) = {}:<br>{}",
      instructions,
      input_str,
      message
    )),
  };
  let text = if results.nonsingular {""} else {
    "<br>(The function does not exist at that point,
    but these are the limits.)"
  };
  let mut expression = input_str.to_string();
  expression = str::replace(&expression, "%5E", "^");
	expression = str::replace(&expression, "%20", ""); // %20 is url encoding of space
  for stri in ["div", "DIV", "d", "D"] {
    expression = str::replace(&expression, stri, "/"); // division operation is a special URL char
  }
  content::Html(format!(
    "{}<br><br><b>results</b> at x = {} for the function f(x) =
    {}:{}<ul><li>f = {}</li><li>f' = {}</li><li>f'' =
    {}</li><li>f''' = {}</li></ul>",
    instructions,
    results.x,
    expression,
    text,
    results.derivs[0],
    results.derivs[1],
    results.derivs[2],
    results.derivs[3],
  ))
}

#[get("/integration/<xi_str>/<xf_str>/<input_str>")]
fn integration_html(
  xi_str: &RawStr,
  xf_str: &RawStr,
  input_str: &RawStr,
) -> content::Html<String> {
  let instructions = integration::page();
  let results = match integration::raw(xi_str, xf_str, input_str) {
    Ok(results) => results,
    Err(message) => return content::Html(format!(
      "{}<br><br><b>result</b> for the integral from x = {} to
      x = {} of the function f(x) = {}:<br>{}",
      instructions,
      xi_str,
      xf_str,
      input_str,
      message
    )),
  };
  let mut expression = input_str.to_string();
  expression = str::replace(&expression, "%5E", "^");
	expression = str::replace(&expression, "%20", ""); // %20 is url encoding of space
  for stri in ["div", "DIV", "d", "D"] {
    expression = str::replace(&expression, stri, "/"); // division operation is a special URL char
  }
  content::Html(format!(
    "{}<br><br><b>result</b>: {} equals the definite integral from x = {}
    to x = {} of the function f(x) = {}.<br>Convergence to an absolute
    accuracy of {} required {} subdivisions.",
    instructions,
    results.integral,
    results.xi,
    results.xf,
    str::replace(&expression, "X", "x"),
    results.epsilon,
    results.subdivisions,
  ))
}

#[get("/root-finding/<xi_str>/<input_str>")]
fn root_finding_html(xi_str: &RawStr, input_str: &RawStr) -> content::Html<String> {
  let instructions = root_finding::page();
  let result = match root_finding::raw(xi_str, input_str) {
    Ok(result) => result,
    Err(message) => return content::Html(format!(
      "{}<br><br><b>result</b> for finding a root of the
      function f(x) = {} after starting at x = {}:<br>{}",
      instructions,
      input_str,
      xi_str,
      message
    )),
  };
  let mut expression = input_str.to_string();
  expression = str::replace(&expression, "%5E", "^");
	expression = str::replace(&expression, "%20", ""); // %20 is url encoding of space
  for stri in ["div", "DIV", "d", "D"] {
    expression = str::replace(&expression, stri, "/"); // division operation is a special URL char
  }
  content::Html(format!(
    "{}<br><br><b>result</b>: {} is the root of the function f(x) = {}
    which is found after starting from x = {}.<br>Bracketing the root
    required {} steps, and convergence to an absolute accuracy of {}
    required {} more steps.",
    instructions,
    result.x,
    str::replace(&expression, "X", "x"),
    result.xi,
    result.bracket_steps,
    result.epsilon,
    result.root_steps,
  ))
}

#[get("/max-finding/<xi_str>/<input_str>")]
fn max_finding_html(xi_str: &RawStr, input_str: &RawStr) -> content::Html<String> {
  let instructions = max_finding::page();
  let result = match max_finding::raw(xi_str, input_str) {
    Ok(result) => result,
    Err(message) => return content::Html(format!(
      "{}<br><br><b>result</b> for the maximum of the
      function f(x) = {} starting at x = {}:<br>{}",
      instructions,
      input_str,
      xi_str,
      message
    )),
  };
  let mut expression = input_str.to_string();
  expression = str::replace(&expression, "%5E", "^");
	expression = str::replace(&expression, "%20", ""); // %20 is url encoding of space
  for stri in ["div", "DIV", "d", "D"] {
    expression = str::replace(&expression, stri, "/"); // division operation is a special URL char
  }
  content::Html(format!(
    "{}<br><br><b>result</b>: ({}, {}) are the coordinates of the local
    maximum of the function f(x) = {} which is found after starting from x = {}.<br>Bracketing the maximum required {} steps, and convergence to an absolute
    accuracy of {} required {} more steps.",
    instructions,
    result.x,
    result.f,
    str::replace(&expression, "X", "x"),
    result.xi,
    result.bracket_steps,
    result.epsilon,
    result.max_steps,
  ))
}

#[get("/ode/<xi_str>/<tf_str>/<nt_str>/<input_str>")]
fn ode_html(
  xi_str: &RawStr,
  tf_str: &RawStr,
  nt_str: &RawStr,
  input_str: &RawStr
) -> content::Html<String> {
  let instructions = ode::page();
  let result = match ode::raw(xi_str, tf_str, nt_str, input_str) {
    Ok(result) => result,
    Err(message) => return content::Html(format!(
      "{}<br><br><b>result</b> for ODE that
      dx/dt = {} if x(0) = {}:<br>{}",
      instructions,
      input_str,
      xi_str,
      message
    )),
  };
  let mut expression = input_str.to_string();
  expression = str::replace(&expression, "%5E", "^");
	expression = str::replace(&expression, "%20", ""); // %20 is url encoding of space
  for stri in ["div", "DIV", "d", "D"] {
    expression = str::replace(&expression, stri, "/"); // division operation is a special URL char
  }
  let mut rows = "".to_string();
  for i in 0..result.xs.len() {
    rows = format!(
      "{}<div>{}</div><div>{}</div>",
      rows,
      (i as f64) * result.tf / (result.nt as f64), result.xs[i],
    );
  }
  rows = format!("
  <div style='display: flex; flex-direction: column;'>
    <div style='display: grid; grid-template-columns: repeat(2, 1fr); width:300px'>
      <div>
        <i>t</I>
      </div>
      <div>
        <i>x</i>
      </div>
    </div>
    <div style='
      height:100px;
      width:300px;
      overflow-y:scroll;
      border-width:1px;
      border-style: solid;
      display: grid;
      grid-template-columns: repeat(2, 1fr);
    '>
      {}
    </div></div>", rows);
  rows = format!(
    "<div style='display: flex;
    justify-content: center;'>{}</div>",
    rows,
  );
  content::Html(format!(
    "{}<br><br><b>result</b>: Solution of the ODE dx/dt = {},
    with the initial condition that x(0) = {}.<br>{}",
    instructions,
    str::replace(&expression, "X", "x"),
    result.xi,
    rows,
  ))
}

#[get("/ode2/<xi_str>/<vi_str>/<tf_str>/<nt_str>/<input_str>")]
fn ode2_html(
  xi_str: &RawStr,
  vi_str: &RawStr,
  tf_str: &RawStr,
  nt_str: &RawStr,
  input_str: &RawStr,
) -> content::Html<String> {
    let instructions = ode2::page();
    let result = match ode2::raw(xi_str, vi_str, tf_str, nt_str, input_str) {
      Ok(result) => result,
      Err(message) => return content::Html(format!(
        "{}<br><br><b>result</b> for 2nd-order ODE that
        d<sup>2</sup>x/dt<sup>2</sup> = {}
        if x(0) = {} and v(0) = {}:<br>{}",
        instructions,
        input_str,
        xi_str,
        vi_str,
        message
      )),
    };

    let mut expression = input_str.to_string();
    expression = str::replace(&expression, "%5E", "^");
	  expression = str::replace(&expression, "%20", ""); // %20 is url encoding of space
    for stri in ["div", "DIV", "d", "D"] {
      expression = str::replace(&expression, stri, "/"); // division operation is a special URL char
    }

    let mut rows = "".to_string();
    for i in 0..result.xs.len() {
      rows = format!(
        "{}<div>{}</div><div>{}</div><div>{}</div>",
        rows,
        (i as f64) * result.tf / (result.nt as f64), result.xs[i], result.vs[i],
      );
    }
    rows = format!("
    <div style='display: flex; flex-direction: column;'>
      <div style='display: grid; grid-template-columns: repeat(3, 1fr); width:500px'>
        <div>
          <i>t</I>
        </div>
        <div>
          <i>x</i>
        </div>
        <div>
          <i>v</i>
        </div>
      </div>
      <div style='
        height:100px;
        width:500px;
        overflow-y:scroll;
        border-width:1px;
        border-style: solid;
        display: grid;
        grid-template-columns: repeat(3, 1fr);
      '>
        {}
      </div></div>", rows);
    rows = format!("<div style='display: flex; justify-content: center;'>{}</div>", rows);
    content::Html(format!("{}<br><br><b>result</b>: Solution of the ODE d<sup>2</sup>x/dt<sup>2</sup> = {}, with the initial conditions that x(0) = {} and that v(0) = {}.<br>{}",
      instructions,
      str::replace(&expression, "X", "x"),
      result.xi,
      result.vi,
      rows,
    ))
}

fn main() {
  rocket::ignite().mount("/", routes![index,
  differentiation_page, differentiation_json, differentiation_html,integration_page, integration_json, integration_html,
  root_finding_page, root_finding_json, root_finding_html,
  max_finding_page, max_finding_json, max_finding_html,
  ode_page, ode_json, ode_html,
  ode2_page, ode2_json, ode2_html]).launch();
}
