use std::fs;
use std::path::Path;

use warp::log::Info;
use warp::path::FullPath;
use warp::reply::Html;
use warp::Filter;

use crate::cmd::cmd_app;

mod cmd;

const DEFAULT_PORT: u16 = 8080;
const WEB_FOLDER: &str = "./";

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let app = cmd_app().get_matches();

	let port = app.value_of("port").and_then(|val| val.parse::<u16>().ok()).unwrap_or(DEFAULT_PORT);

	let content = warp::fs::dir(WEB_FOLDER);
	let root = warp::get()
		.and(warp::path::end())
		.and(warp::fs::file(format!("{}/index.html", WEB_FOLDER)));
	let static_site = content.or(root).or(folder_content_filter());

	let routes = static_site.with(warp::log::custom(log_req));

	println!("starting server at http://localhost:{}/", port);

	warp::serve(routes).run(([127, 0, 0, 1], port)).await;

	Ok(())
}

pub fn folder_content_filter() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::any().and(warp::path::full()).and_then(folder_content_handler)
}

async fn folder_content_handler(full_path: FullPath) -> Result<Html<String>, warp::Rejection> {
	// IMPORTANT - trim all start '/' to make sure all is relatively to './'
	let uri = full_path.as_str().trim_start_matches('/');

	let root_path = Path::new(WEB_FOLDER).to_path_buf();
	let dir_path = root_path.join(uri);

	let mut html = String::new();

	let paths = fs::read_dir(&dir_path);

	match paths {
		Ok(paths) => {
			for path in paths.into_iter() {
				if let Some(path) = path.ok().and_then(|v| v.path().to_str().map(|s| s.to_string())) {
					let uri = &path[WEB_FOLDER.len()..];
					let href = format!("/{}", uri);
					html.push_str(&format!(r#"<a href="{}">{}</a><br />"#, href, uri));
				}
			}
		}
		Err(_) => html.push_str(&format!("Cannot read dir of '{}'", dir_path.to_string_lossy())),
	}

	let html = warp::reply::html(html);

	Ok(html)
}

fn log_req(info: Info) {
	println!(
		" {} {} {} ({}ms)",
		info.method(),
		info.status(),
		info.path(),
		info.elapsed().as_micros() as f64 / 1000.0,
	);
}
