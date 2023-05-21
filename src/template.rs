/////
/////
/////

pub struct PromptOptions<'a> {
    pub title: &'a str,
    pub subtitle: &'a str,
    pub badge: Badge<'a>,
    pub sections: Vec<String>,
    pub image_count: u8,
}

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
    const MIT: &str = "[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)";
    const APACHE: &str ="[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)";
    const MOZILLA: &str = "[![License: MPL 2.0](https://img.shields.io/badge/License-MPL_2.0-brightgreen.svg)](https://opensource.org/licenses/MPL-2.0)";
    const GNU: &str = "[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)";

    pub fn match_str(kind: &str) -> LicenseBadge {
        match kind.to_lowercase().trim() {
            "mit" => LicenseBadge::MIT,
            "apache" => LicenseBadge::Apache,
            "mozilla" => LicenseBadge::Mozilla,
            "gnu" => LicenseBadge::GNU,
            _ => LicenseBadge::Default,
        }
    }

    pub fn generate(kind: LicenseBadge) -> Self {
        match kind {
            LicenseBadge::MIT => Badge(Self::MIT),
            LicenseBadge::Apache => Badge(Self::APACHE),
            LicenseBadge::Mozilla => Badge(Self::MOZILLA),
            LicenseBadge::GNU => Badge(Self::GNU),
            LicenseBadge::Default => Badge(Self::MIT),
        }
    }
}

fn title(text: &str) -> String {
    format!(r"# {text}")
}

fn subtitle(text: &str) -> String {
    format!(r"### *{text}*")
}

fn description() -> &'static str { 
        r"
### __Description__

Love you, then bite you where is my slave? I'm getting hungry caticus cuteicus yet stare at owner accusingly then wink or love you, then bite you.
"
}

fn section(name: &str) -> String {
    format!(
        r"
## _{name}_
> `CONTENT`

---
"
    )
}

fn sections(sect_vec: Vec<String>) -> String {
    sect_vec.iter().map(move |header| section(header)).collect()
}

pub fn toc(sect_vec: Vec<String>) -> String {
    format!(
        r"
## Table of Contents
{}
---
---
---
",
        sect_vec
            .iter()
            .map(|header| {
                let link = header.replace(" ", "-");
                format!(
                    r"
- __*::: [{}](#{}) :::*__
        ",
                    header,
                    link.to_lowercase()
                )
            })
            .collect::<String>()
    )
}

fn default_sections() -> Vec<String> {
    vec![
        "Installation".to_string(),
        "Usage".to_string(),
        "Contributing".to_string(),
        "Tests".to_string(),
        "Questions".to_string(),
    ]
}

fn images(count: u8) -> String {
    (0..count).fold(String::new(), |acc, _| {
        acc + "![Image Title](https://image-link.com/image.png)\n"
    })
}

pub fn default(badge: &str) -> String {
    format!(
        r"
{}
{}     
{} 
---
{}
{}
---
---
---
{}

{}
",
        title("Super Cool Project"),
        badge.to_owned() + "\n",
        subtitle("Very cool, and blazingly fast"),
        description(), 
        images(3),
        toc(default_sections()),
        sections(default_sections())
    )
}

pub fn prompted(options: PromptOptions) -> String {
    format!(
        r"
{}
{}
{}  
---
{}
{}
---
---
---
{}

{}
",
        title(options.title),
        options.badge.0.to_owned() + "\n",
        subtitle(options.subtitle),
        section("Description"),
        images(options.image_count),
        toc(options.sections.clone()),
        sections(options.sections)
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
