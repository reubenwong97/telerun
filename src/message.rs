use crate::models::Run;
use askama::Template;

#[derive(Template)]
#[template(path = "list_runs.html", print = "all")]
struct ListRunTemplate<'a> {
    runs: &'a Vec<Run>,
}

pub fn list_runs(runs: Option<Vec<Run>>) -> String {
    if let Some(runs) = runs {
    } else {
        "No runs in database.".into()
    }
}
