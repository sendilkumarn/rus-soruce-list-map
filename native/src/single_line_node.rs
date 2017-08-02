use helper;
use base64_vlq::encode;
use source_node::SourceNode;
use mappings_context::MappingsContext;

#[derive(Clone)]
pub struct SingleLineNode {
    pub generated_code: String,
    pub original_source: String,
    pub source: String,
    pub number_of_lines: usize,
    pub ends_with_new_line: bool,
    pub line: usize,
}

impl SingleLineNode {
    pub fn new(generated_code: &str, source: String, original_source: String, line: usize) -> Self {
        SingleLineNode {
            // TODO check for efficient and good perf code here
            generated_code: String::from(generated_code),
            original_source: original_source,
            source: source,
            line: line,
            number_of_lines: helper::number_of_lines(generated_code),
            ends_with_new_line: String::from(generated_code).ends_with("/n"),
        }
    }

    pub fn get_generated_code(&self) -> &str {
        &self.generated_code
    }

    pub fn add_generated_code(&mut self, code: &str) {
        self.generated_code += code;
        self.number_of_lines += helper::number_of_lines(code);
        self.ends_with_new_line = code.ends_with("\n");
    }

    pub fn get_mappings(&self, ref mut mappings_context: &mut MappingsContext) -> String {
        if self.generated_code.is_empty() {
            String::from("")
        } else {
            let line_mapping = ";AAAA";
            let lines = self.number_of_lines;
            let source_index =
                mappings_context.ensure_source(self.source.clone(), self.original_source.clone());
            let mut mappings = String::from("A");
            if mappings_context.unfinished_generated_line != 0 {
                mappings = String::from(",");
                // encode(mappings_context.unfinished_generated_line as i64, &mut buffer);
                mappings += &encode(mappings_context.unfinished_generated_line as i32);
            }
            mappings += &encode((source_index - mappings_context.current_source) as i32);
            mappings += &encode((self.line - mappings_context.current_original_line) as i32);
            mappings += "A"; // original column 0
            mappings_context.current_source = source_index;
            mappings_context.current_original_line = self.line;

            let unfinished_generated_line = helper::get_unfinished_lines(&self.generated_code);
            mappings_context.unfinished_generated_line = unfinished_generated_line;
            mappings += &line_mapping.repeat(lines - 1);
            if unfinished_generated_line == 0 {
                mappings += ";";
            } else {
                if lines != 0 {
                    mappings += line_mapping;
                }
            }
            mappings
        }
    }

    // check
    pub fn get_normalized_nodes(&self) -> Vec<SingleLineNode> {
        let mut v: Vec<SingleLineNode> = vec![];
        v.push(self.clone());
        v
    }

    pub fn map_generated_code(&self, f: &Fn(String) -> String) -> SingleLineNode {
        let generated_code = f(self.clone().generated_code);
        println!("{}", generated_code);
        SingleLineNode::new(&generated_code,
                            self.source.clone(),
                            self.original_source.clone(),
                            self.line)
    }

    pub fn merge(&mut self, other_node: SingleLineNode) -> Option<&SingleLineNode> {
        self.merge_single_line_node(other_node)
    }

    fn merge_single_line_node(&mut self, other_node: SingleLineNode) -> Option<&SingleLineNode> {
        if self.source == other_node.source && self.original_source == other_node.original_source {
            if self.line == other_node.line {
                self.generated_code += &other_node.generated_code;
                self.number_of_lines += other_node.number_of_lines;
                self.ends_with_new_line = other_node.ends_with_new_line;
                Some(self)
            } else if self.line + 1 == other_node.line && self.ends_with_new_line &&
                      self.number_of_lines == 1 &&
                      other_node.number_of_lines <= 1 {
                // Todo check how to return this value
                let this = self.clone();
                SourceNode::new(this.generated_code + &other_node.generated_code,
                                this.source,
                                this.original_source,
                                this.line);
                None
            } else {
                None
            }
        } else {
            None
        }
    }

    fn add_single_line_node(&mut self, other_node: SingleLineNode) -> &SingleLineNode {
        self.generated_code += &other_node.generated_code;
        self.number_of_lines += other_node.number_of_lines;
        self.ends_with_new_line = other_node.ends_with_new_line;
        self
    }
}
