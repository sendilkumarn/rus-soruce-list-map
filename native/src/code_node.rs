use helper;
use mappings_context::MappingsContext;

#[derive(Clone)]
pub struct CodeNode {
    pub generated_code: String,
}

impl CodeNode {
    pub fn new(generated_code: String) -> Self {
        CodeNode { generated_code: generated_code }
    }

    pub fn generated_code(&self) -> &str {
        &self.generated_code
    }

    pub fn get_mappings(&self, ref mut mappings_context: &mut MappingsContext) -> String {
        let lines = helper::number_of_lines(&self.generated_code);
        let mapping = "?".repeat(lines + 1);

        if lines > 0 {
            mappings_context.unfinished_generated_line =
                helper::get_unfinished_lines(&self.generated_code);
            if mappings_context.unfinished_generated_line > 0 {
                mapping + "A"
            } else {
                mapping
            }
        } else {
            let prev_unfinished = mappings_context.unfinished_generated_line;
            mappings_context.unfinished_generated_line +=
                helper::get_unfinished_lines(&self.generated_code);
            if prev_unfinished == 0 && mappings_context.unfinished_generated_line > 0 {
                String::from("A")
            } else {
                String::from("")
            }
        }
    }

    pub fn add_generated_code(&mut self, generated_code: &str) {
        self.generated_code += generated_code;
    }

    pub fn map_generated_code(&self, f: &Fn(String) -> String) -> CodeNode {
        let generated_code = f(self.clone().generated_code);
        CodeNode::new(generated_code)
    }

    pub fn get_normalized_nodes(&self) -> Vec<CodeNode> {
        let mut v: Vec<CodeNode> = vec![];
        v.push(self.clone());
        v
    }

    pub fn merge(&mut self, other_node: CodeNode) -> &CodeNode {
        self.generated_code += &other_node.generated_code;
        self
    }
}
