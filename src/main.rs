use pathdiff::diff_paths;
use std::format as f;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use warp::log::Info;
use warp::path::FullPath;
use warp::reply::Html;
use warp::Filter;

use crate::tmpl::{HTML_DIR_LIST_END, HTML_DIR_LIST_START};

use crate::cmd::cmd_app;
use crate::xts::AsString;

mod cmd;
mod tmpl;
mod xts;

const DEFAULT_PORT: u16 = 8080;
const DEFAULT_WEB_FOLDER: &str = "./";

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let app = cmd_app().get_matches();

	// --- Get the port
	let port = app
		.value_of("port")
		.and_then(|val| val.parse::<u16>().ok())
		.unwrap_or(DEFAULT_PORT);

	// --- Get the root directory path
	let root_dir = app
		.value_of("dir")
		.map(|v| v.to_owned())
		.unwrap_or_else(|| DEFAULT_WEB_FOLDER.to_owned());
	println!("->> DIR PATH >>> {}", root_dir);

	let root_dir = Path::new(&root_dir).to_path_buf();
	let root_dir = Arc::new(root_dir);

	let special_filter = with_path_type(root_dir.clone()).and_then(special_file_handler);

	let warp_dir_filter = warp::fs::dir(root_dir.to_path_buf());

	let routes = special_filter.or(warp_dir_filter);

	// add the log
	let routes = routes.with(warp::log::custom(log_req));

	println!("Starting server at http://localhost:{}/", port);

	warp::serve(routes).run(([127, 0, 0, 1], port)).await;

	Ok(())
}

struct PathInfo {
	root_dir: Arc<PathBuf>,
	target_path: PathBuf,
}
enum SpecialPath {
	Dir(PathInfo),
	ExtLessFile(PathInfo),
	NotSpecial,
}

fn with_path_type(root_dir: Arc<PathBuf>) -> impl Filter<Extract = (SpecialPath,), Error = std::convert::Infallible> + Clone {
	warp::any().and(warp::path::full()).map(move |full_path: FullPath| {
		let web_path = full_path.as_str().trim_start_matches('/');
		let target_path = root_dir.join(web_path);

		let path_info = PathInfo {
			root_dir: root_dir.clone(),
			target_path: target_path,
		};

		if path_info.target_path.is_dir() {
			SpecialPath::Dir(path_info)
		} else if path_info.target_path.is_file() && path_info.target_path.extension().is_none() {
			SpecialPath::ExtLessFile(path_info)
		} else {
			SpecialPath::NotSpecial
		}
	})
}

async fn special_file_handler(special_path: SpecialPath) -> Result<Html<String>, warp::Rejection> {
	match special_path {
		SpecialPath::Dir(path_info) => {
			let PathInfo { root_dir, target_path } = path_info;
			let mut html = String::new();

			let paths = fs::read_dir(&target_path);
			match paths {
				Ok(paths) => {
					for path in paths.into_iter() {
						if let Some(path) = path.ok().map(|v| v.path()) {
							if let Some(diff) = diff_paths(&path, root_dir.as_ref()).x_as_string() {
								let disp = path.file_name().and_then(|s| s.to_str()).unwrap_or("unknown");
								let suffix = if path.is_dir() { "/" } else { "" };
								let href = format!("/{}", diff);
								html.push_str(&format!(r#"<a href="{}">{}{suffix}</a>"#, href, disp));
							}
						}
					}
				}
				Err(_) => html.push_str(&format!("Cannot read dir of '{}'", target_path.to_string_lossy())),
			}

			let html = f!("{HTML_DIR_LIST_START}{html}{HTML_DIR_LIST_END}");

			Ok(warp::reply::html(html))
		}
		SpecialPath::ExtLessFile(path_info) => {
			// FIXME: Remove the unwrap
			let html = fs::read_to_string(path_info.target_path).unwrap();
			Ok(warp::reply::html(html))
		}
		// When not special, return not found in this handler, so that the default warp::dir
		// filter can take over.
		SpecialPath::NotSpecial => Err(warp::reject::not_found()),
	}
}

fn log_req(info: Info) {
	println!(
		" {} {} {} ({}ms)",
		info.method(),
		info.status(),
		info.path(),
		info.elapsed().as_micros() as f64 / 1000.
	);
}
