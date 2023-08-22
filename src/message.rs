use crate::models::Run;
use askama::Template;
use std::fmt;
use std::ops;

struct RunDisplay(Run);

impl ops::Deref for RunDisplay {
    type Target = Run;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for RunDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // No longer need to access via self.0.id, since we deref
        // a RunDisplay into a Run!
        write!(
            f,
            "{} {} {} {}",
            self.id,
            self.distance,
            self.run_datetime
                .map(|x| x.to_string())
                .unwrap_or("NULL".to_string()),
            self.user_id
        )
    }
}

#[derive(Template)]
#[template(path = "list_runs.html")]
struct ListRunTemplate<'a> {
    runs: &'a Vec<RunDisplay>,
}

pub fn list_runs(runs: Option<Vec<Run>>) -> String {
    if let Some(runs) = runs {
        let run_displays: Vec<RunDisplay> = runs.into_iter().map(|run| RunDisplay(run)).collect();
        let run_template = ListRunTemplate {
            runs: &run_displays,
        };

        format!("{}", run_template.render().unwrap())
    } else {
        "No runs in database.".into()
    }
}

#[cfg(test)]
mod tests {
    use sqlx::types::chrono;

    use super::*;

    #[test]
    fn list_runs_template() {
        let runs = vec![
            Run {
                id: 1,
                distance: 1.,
                run_datetime: chrono::NaiveDateTime::from_timestamp_opt(61, 0),
                user_id: 1,
            },
            Run {
                id: 2,
                distance: 2.,
                run_datetime: chrono::NaiveDateTime::from_timestamp_opt(82, 0),
                user_id: 2,
            },
        ];
        let render = list_runs(Some(runs));
        // TODO: i actually dont want this kind of html templates anyway
        let ans = "
  1. 1 1 1970-01-01 00:01:01 1

  2. 2 2 1970-01-01 00:01:22 2
";
        assert_eq!(render, ans);
    }
}
