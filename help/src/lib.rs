use std::cell::RefCell;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct HelpMessage {
    name: String,
    description: String,
    options: Vec<HelpMessageOption>,
    subcommands: Vec<HelpMessage>,

    print_recursion_level: RefCell<usize>,
}

impl HelpMessage {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            description: String::new(),
            options: Vec::new(),
            subcommands: Vec::new(),
            print_recursion_level: RefCell::new(0),
        }
    }

    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = name.to_string();
        self
    }

    pub fn description(&mut self, description: &str) -> &mut Self {
        self.description = description.to_string();
        self
    }

    pub fn add_option(&mut self, opt: HelpMessageOption) -> &mut Self {
        self.options.push(opt);
        self
    }

    pub fn add_subcommand(&mut self, cmd: HelpMessage) -> &mut Self {
        self.subcommands.push(cmd);
        self
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct HelpMessageOption {
    name: String,
    kind: String,
    description: String,
    required: bool,
}

impl HelpMessageOption {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = name.to_string();
        self
    }

    pub fn kind(&mut self, kind: &str) -> &mut Self {
        self.kind = kind.to_string();
        self
    }

    pub fn description(&mut self, description: &str) -> &mut Self {
        self.description = description.to_string();
        self
    }

    pub fn required(&mut self, required: bool) -> &mut Self {
        self.required = required;
        self
    }
}

impl std::fmt::Display for HelpMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut recursion_level = self.print_recursion_level.borrow_mut();
        let mut indentation = "    ".to_string();

        for _ in 0..*recursion_level {
            indentation.push_str("        ");
        }

        let mut m = format!("{} -- {}", self.name, self.description);

        if !self.options.is_empty() {
            m.push_str(&format!("\n{indentation}Options:"))
        }

        for opt in &self.options {
            let required = if opt.required { "Required" } else { "Optional" };
            m.push_str(&format!(
                "\n{indentation}    {}: {}\t{} ({required})",
                opt.name, opt.kind, opt.description
            ))
        }

        if !self.subcommands.is_empty() {
            m.push_str(&format!("\n{indentation}Subcommands:\n"));
        }

        for subcommand in &self.subcommands {
            *recursion_level += 1;
            m.push_str(&format!("{indentation}{indentation}{subcommand}\n"));
            *recursion_level -= 1;
        }
        write!(f, "{m}")
    }
}
