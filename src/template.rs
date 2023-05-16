use std::fs::File;
use std::io::{LineWriter, Write};

#[derive(Debug)]
pub struct Badge<'a>(pub &'a str);

pub enum LicenseBadge {
    MIT,
    Apache,
    Mozilla,
    GNU,
    Default,
}

impl<'a> Badge<'a> {
    pub fn match_str(kind: &str) -> LicenseBadge {
        match kind.trim() {
            "MIT" => LicenseBadge::MIT,
            "Apache" => LicenseBadge::Apache,
            "Mozilla" => LicenseBadge::Mozilla,
            "GNU" => LicenseBadge::GNU,
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

pub fn toc() -> &'static str {
    r"
## Description  
> Describe the Application

---
---
---

## Table of Contents  

*::: [Installation](#installation) :::*

*::: [Usage](#usage) :::*

*::: [Contributing](#contributing) :::*

*::: [Tests](#tests) :::*

*::: [Questions](#questions) :::*

## Installation  
> Instructions go here  

---

## Usage  
> How to use app/repo

---

## Contributing
> Special mentions / how to contribute

---

## Tests  
> How to run tests / performance explanation

---

## Questions  
> How to get in contact
"
}

pub fn default(badge: &str) -> String {
    format!(
        r"
# Super Cool Project  

### Very cool, and blazingly fast!
      
{}

{}
",
        badge,
        toc()
    )
}

fn spacer(writer: &mut LineWriter<File>) {
    writer
        .write_all("___\n".as_bytes())
        .expect("Writer should have written spacer characters");
}

fn spacers(writer: &mut LineWriter<File>) {
    spacer(writer);
    spacer(writer);
    spacer(writer);
}
