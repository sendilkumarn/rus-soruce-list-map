use std::collections::HashMap;

#[derive(Clone)]
pub struct MappingsContext {
    pub sources_indices: HashMap<String, usize>,
    pub sources_content: HashMap<String, String>,
    pub has_source_content: bool,
    pub current_original_line: usize,
    pub current_source: usize,
    pub unfinished_generated_line: usize,
}

impl MappingsContext {
    pub fn new() -> Self {
        MappingsContext {
            sources_indices: HashMap::new(),
            sources_content: HashMap::new(),
            has_source_content: false,
            current_original_line: 1,
            current_source: 0,
            unfinished_generated_line: 0,
        }
    }

    // TODO implement this method
    pub fn ensure_source(&mut self, src: String, original_source: String) -> usize {
        let sources_indices = self.sources_indices.clone();
        match sources_indices.get(&src) {
            Some(si) => *si,
            None => {
                let idx = self.sources_indices.len();
                self.sources_indices.insert(src.clone(), idx);
                self.sources_content.insert(src, original_source);
                idx
            }
        }
    }

    // TODO implement this method
    // possible return -> (Vec<source>, Vec<source>)
    pub fn get_arrays(&self) -> Src {
        let mut sources = Vec::new();
        let mut sources_content = Vec::new();
        for (key, value) in self.sources_content.clone() {
            sources.push(value);
            sources_content.push(key);
        }
        Src::new(sources, sources_content)
    }
}

#[allow(dead_code)]
pub struct Src {
    pub sources: Vec<String>,
    pub sources_content: Vec<String>,
}

impl Src {
    pub fn new(sources: Vec<String>, sources_content: Vec<String>) -> Self {
        Src {
            sources: sources,
            sources_content: sources_content,
        }
    }
}
