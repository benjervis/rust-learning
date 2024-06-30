use std::iter::Peekable;

type Chars<'a> = Peekable<std::str::Chars<'a>>;

struct Diagnostic {
    message: String,
    line: u32,
    column: u32,
}

struct GraphAST {
    root: Node,
}

struct GraphNode {}
enum Node {
    Graph(GraphNode),
}

pub fn parse(source: Chars) -> Result<GraphAST, Diagnostic> {
    let parser = Parser { source };

    parser.run()
}

struct Parser<'a> {
    source: Chars<'a>,
}

impl<'a> Parser<'a> {
    fn run(&self) -> Result<GraphAST, Diagnostic> {
        todo!()
    }

    fn read_word(&mut self) -> Result<String, Diagnostic> {
        let mut result = String::new();
        while let Some(char) = self.source.next_if(|c| !c.is_whitespace()) {
            result.push(char);
        }

        if result.is_empty() {
            return Err(self.make_diagnostic("Expected word, received nothing"));
        }

        Ok(result)
    }

    fn make_diagnostic(&self, message: &str) -> Diagnostic {
        Diagnostic {
            message: message.to_string(),
            line: 0,
            column: 0,
            // line: self.source.line,
            // column: self.source.column,
        }
    }
}

#[cfg(test)]
mod test {
    use super::parse;

    #[test]
    fn should_work() {
        let input: String = "graph MyGraph {}".to_string();
        let result = parse(input.chars().peekable());

        assert(matches!(result, Ok()));
    }
}
