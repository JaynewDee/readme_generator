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
format!(r"
# Super Cool Project  

### Very cool, and blazingly fast!
      
{}

{}
",
        badge,
        toc()
    )
}
