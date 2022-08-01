mod html_dir_list;

pub use html_dir_list::*;

pub const JS_LIVE_CONTENT: &str = include_str!("./_webhere_live.js");

pub const JS_LIVE_SCRIPT_TAG: &str = "\n<script src=\"/_webhere_live.js\"></script>";
