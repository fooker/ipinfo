use std::io::Write;

use anyhow::Result;

use crate::details::Details;

mod human;
mod json;

pub enum Output {
    HUMAN,
    JSON,
}

pub fn display(w: impl Write, output: Output, details: Details) -> Result<()> {
    return match output {
        Output::HUMAN => human::display(w, details),
        Output::JSON => json::display(w, details),
    };
}
