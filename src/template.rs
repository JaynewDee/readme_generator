use std::fs::File;
use std::io::{LineWriter, Write};

#[derive(Debug)]
pub struct Badge<'a>(pub &'a str);

#[derive(PartialEq)]
pub enum LicenseBadge {
    MIT,
    Apache,
    Mozilla,
    GNU,
    Default,
}

impl<'a> Badge<'a> {
    pub fn match_str(kind: &str) -> LicenseBadge {
        match kind.trim().to_lowercase().as_str() {
            "mit" => LicenseBadge::MIT,
            "apache" => LicenseBadge::Apache,
            "mozilla" => LicenseBadge::Mozilla,
            "gnu" => LicenseBadge::GNU,
            _ => LicenseBadge::Default,
        }
    }
    pub fn generate(kind: LicenseBadge) -> Self {
        match kind {
            LicenseBadge::MIT => Badge("[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)"),
            LicenseBadge::Apache => Badge("[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)"),
            LicenseBadge::Mozilla => Badge("[![License: MPL 2.0](https://img.shields.io/badge/License-MPL_2.0-brightgreen.svg)](https://opensource.org/licenses/MPL-2.0)"),
            LicenseBadge::GNU => Badge("[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)"),
            LicenseBadge::Default => Badge("[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)")
        }
    }
}
fn title(text: &str) -> String {
    format!(r"# {text}")
}

fn subtitle(text: &str) -> String {
    format!(r"### {text}")
}
fn section(name: &str, content: &str) -> String {
    format!(
        r"
## {name}
> {content}
"
    )
}

pub fn toc() -> &'static str {
    r"
## Table of Contents  

*::: [Installation](#installation) :::*

*::: [Usage](#usage) :::*

*::: [Contributing](#contributing) :::*

*::: [Tests](#tests) :::*

*::: [Questions](#questions) :::*
"
}

pub fn default(badge: &str) -> String {
    format!(
        r"
{}
{}     
{}
{}
---
---
---
{}

{}
{}
{}
{}
{}
",
        title("Super Cool Project"),
        subtitle("Very cool, and blazingly fast"),
        badge,
        section("Description", "Describe the project / application"),
        toc(),
        section("Installation", "Explain installation procedure"),
        section("Usage", "Describe how the app is meant to be used"),
        section("Contributing", "Explain how to contribute to the project"),
        section("Tests", "Test runner instructions"),
        section("Questions", "Contact the developers")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_kind() {
        let badge = Badge::match_str(" MIT");
        assert!(badge == LicenseBadge::MIT);

        let badge = Badge::match_str(" aPAcHE\n ");
        assert!(badge == LicenseBadge::Apache);

        let badge = Badge::match_str("gOOsecake ");
        assert!(badge == LicenseBadge::Default)
    }
}
