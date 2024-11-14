pub const INSTRUCTIONS: &str = "WELCOME TO MY CALCULUS APP";

const GITHUB: &str = "https://pknipp.github.io";
const HEROKU: &str = "https://basic-calculus.herokuapp.com";
const LINKS: [[&str; 4]; 8] = [
	[GITHUB, "/math", "back to", " math APIs page"],
	[HEROKU, "", "back to", " calculus page"],
	[HEROKU, "/differentiation", "differentiation", ""],
	[HEROKU, "/integration", "integration", ""],
	[HEROKU, "/root-finding", "root-finding", ""],
	[HEROKU, "/max-finding", "max-finding", ""],
	[HEROKU, "/ode", "1st order", " differential equations"],
	[HEROKU, "/ode2", "2nd order", " differential equations"],
];

pub fn general_page() -> String {format!(
	"<p align=center>{}</p><p align=center>{}</p>",
	INSTRUCTIONS,
	links(1),
)}

pub fn links(n: i32) -> String {
	let mut links = "".to_string();
	for j in 0..8 {
		let i = j as usize;
		if i != n as usize {
			links = format!("{}
				<a href='{}{}'>{}</a>{}<br>",
				links, LINKS[i][0], LINKS[i][1], LINKS[i][2], LINKS[i][3]
			);
		}
	}
	links = format!(
		"creator:&nbsp;<a
			href='https://pknipp.github.io/'
			target='_blank'
			rel='noopener noreferrer'
		>
			Peter Knipp
		</a>
		<br/>
		repo:&nbsp;<a
			href='https://github.com/pknipp/calculus'
			target='_blank'
			rel='noopener noreferrer'
		>
			github.com/pknipp/calculus
		</a>
		<br/><br/>{}"	,
		links,
	);
	links
}
