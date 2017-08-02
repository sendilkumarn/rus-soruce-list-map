use helper;
use base64_vlq::encode;
use single_line_node::SingleLineNode;
use mappings_context::MappingsContext;

#[derive(Clone)]
pub struct SourceNode {
    pub generated_code: String,
    pub original_source: String,
    pub source: String,
    pub starting_line: usize,
    pub number_of_lines: usize,
    pub ends_with_new_line: bool,
    pub line: usize,
}

impl SourceNode {
    pub fn new(generated_code: String,
               original_source: String,
               source: String,
               line: usize)
               -> Self {
        SourceNode {
            generated_code: generated_code.clone(),
            source: source,
            original_source: original_source,
            line: line,
            number_of_lines: 1,
            starting_line: 1,
            ends_with_new_line: generated_code.ends_with("\n"),
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
                mappings += &encode(mappings_context.unfinished_generated_line as i32);
            }
            mappings += &encode((source_index - mappings_context.current_source) as i32); // source index
            mappings += &encode((self.line - mappings_context.current_original_line) as i32); // original line index
            mappings += "A"; // original column 0

            mappings_context.current_source = source_index;
            mappings_context.current_original_line = self.starting_line + lines - 1;

            let unfinished_generated_line = helper::get_unfinished_lines(&self.generated_code);
            mappings_context.unfinished_generated_line = unfinished_generated_line;
            mappings += &line_mapping.repeat(lines);
            if unfinished_generated_line == 0 {
                mappings += ";";
            } else {
                if lines != 0 {
                    mappings += line_mapping;
                }
                mappings_context.current_original_line += 1;
            }
            mappings
        }
    }

    // TODO return error
    pub fn map_generated_code(&self, f: &Fn(String) -> String) -> Option<SourceNode> {
        None
    }

    // check
    pub fn get_normalized_nodes(&self) -> Vec<SingleLineNode> {
        let mut results = Vec::new();
        let mut current_line = self.starting_line;
        // let ref generated_code = self.generated_code;
        // let mut index = 0;
        // let index_end = self.generated_code.len();
        for line in self.generated_code.lines() {
            results.push(SingleLineNode::new(&line.to_owned(),
                                             self.source.clone(),
                                             self.original_source.clone(),
                                             current_line));
            current_line += 1;
        }

        results
    }

    pub fn merge(&mut self, other_node: SourceNode) -> &SourceNode {
        self.generated_code += &other_node.generated_code;
        self
    }

    fn merge_source_node(&mut self, other_node: SourceNode) -> Option<&SourceNode> {
        if self.source == other_node.source && self.ends_with_new_line &&
           self.starting_line + self.number_of_lines == other_node.starting_line {
            self.generated_code += &other_node.generated_code;
            self.number_of_lines += other_node.number_of_lines;
            self.ends_with_new_line = other_node.ends_with_new_line;
            Some(self)
        } else {
            None
        }
    }

    fn merge_single_line_node(&mut self, other_node: SingleLineNode) -> Option<&SourceNode> {
        if self.source == other_node.source && self.ends_with_new_line &&
           self.starting_line + self.number_of_lines == other_node.line &&
           other_node.number_of_lines <= 1 {
            self.add_single_line_node(other_node);
            Some(self)
        } else {
            None
        }

    }

    fn add_single_line_node(&mut self, other_node: SingleLineNode) -> &SourceNode {
        self.generated_code += &other_node.generated_code;
        self.number_of_lines += other_node.number_of_lines;
        self.ends_with_new_line = other_node.ends_with_new_line;
        self
    }
}
