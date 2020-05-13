mod github;
mod git;

use std::env;
use itertools::Itertools;
use regex::Regex;

struct GlobPatternBuilder<'a> {
    rules: Vec<&'a str>,
}

impl<'a> GlobPatternBuilder<'a> {
    fn new() -> Self {
        GlobPatternBuilder {
            rules: Vec::new(),
        }
    }
    fn add(&mut self, rule: &'a str) -> &mut Self {
        self.rules.push(rule);
        self
    }

    fn build(&self) -> regex::Regex {
        // **/*.c => ([^/\]+[\/])+
        let operator_pattern = Regex::new("(\\*\\*[/\\\\])|(\\*)").unwrap();
        let expression =
            self.rules.iter()
                // .map(|x| x.replace("**/","([\\\\/]?[^/\\\\]+[\\\\/]?)*").replace("*", "[^/\\\\]+"))
                .map(|x| operator_pattern.captures_iter(x).)
                .join("|");
        regex::Regex::new(expression.as_str()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::GlobPatternBuilder;

    #[test]
    fn test_glob_pattern_builder() {
        let mut builder = GlobPatternBuilder::new();
        builder.add("**/*.go").add("**/*.ts").add("hello/**/*.c");
        let pattern = builder.build();
        #[derive(Debug)]
        struct Case<'a> {
            input: &'a str,
            should_match: bool,
        }
        let cases = vec![
            Case{
                input: "main.go",
                should_match: true,
            },
            Case{
                input: "main.c",
                should_match: false,
            },
            Case {
                input: "hello/main.c",
                should_match: true,
            },
            Case {
                input: "hello/src/main.c",
                should_match: true,
            },
            Case {
                input:"deep/deep/deep/1.ts",
                should_match: true,
            },
        ];
        for c in cases {
            assert_eq!(c.should_match, pattern.is_match(c.input), "case => {:?} pattern => {}", c, pattern);
        }
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 3 {
        println!("Usage: action-change-filter '<filter expression>' command args0 args1 ...");
        return;
    }

}
