use code_node::CodeNode;
use source_node::SourceNode;
use single_line_node::SingleLineNode;
use mappings_context::MappingsContext;

#[derive(Clone)]
pub struct SourceListMap {
    pub generated_code: String,
    pub source: String,
    pub original_source: String,
    pub children: Vec<ChildNode>,
}

impl SourceListMap {
    pub fn new(generated_code: String,
               source: String,
               original_source: String,
               children: Option<Vec<ChildNode>>)
               -> Self {
        SourceListMap {
            source: source,
            generated_code: generated_code,
            original_source: original_source,
            children: match children {
                Some(c) => c,
                None => Vec::new(),
            },
        }
    }


    pub fn new_from_code(&mut self, generated_code: Gc) {
        match generated_code {
            Gc::CodeVec(cv) => self.children = cv,
            Gc::Code(c) => {
                self.children = Vec::new();
                if c.len() != 0 && self.source.len() != 0 {
                    self.add_val(ChildNode::Gcstring(c))
                }
            }
        }
    }

    pub fn add_val(&mut self, generated_code: ChildNode) {
        let source = self.source.clone();
        let original_source = self.original_source.clone();
        match generated_code {
            ChildNode::Gcstring(generated_code_string) => {
                if source.len() != 0 {
                    self.children
                        .push(ChildNode::Gcsourcenode(SourceNode::new(generated_code_string,
                                                                      original_source,
                                                                      source,
                                                                      1)));
                } else if self.children.len() > 0 {
                    let mut child = self.children.get(self.children.len() - 1).unwrap().clone();
                    match child {
                        ChildNode::Gccodenode(ref mut cn) => {
                            cn.add_generated_code(&generated_code_string)
                        }
                        _ => {
                            self.children
                                .push(ChildNode::Gccodenode(CodeNode::new(generated_code_string)))
                        }
                    }
                } else {
                    self.children.push(ChildNode::Gccodenode(CodeNode::new(generated_code_string)));
                }
            }
            ChildNode::Gcsourcenode(sn) => self.children.push(ChildNode::Gcsourcenode(sn)),
            ChildNode::Gcsourcelistmap(slm) => {
                for child in slm.children {
                    self.children.push(child);
                }
            }
            _ => {}
        }
    }

    pub fn add(&mut self, generated_code: ChildNode, source: String, original_source: String) {

        match generated_code {
            ChildNode::Gcstring(generated_code_string) => {
                if source.len() != 0 {
                    self.children
                        .push(ChildNode::Gcsourcenode(SourceNode::new(generated_code_string,
                                                                      source,
                                                                      original_source,
                                                                      1)));
                    println!("{}", self.children.len());
                } else if self.children.len() > 0 {
                    let mut child = self.children.get(self.children.len() - 1).unwrap().clone();
                    match child {
                        ChildNode::Gccodenode(ref mut cn) => {
                            cn.add_generated_code(&generated_code_string)
                        }
                        _ => {
                            self.children
                                .push(ChildNode::Gccodenode(CodeNode::new(generated_code_string)))
                        }
                    }
                } else {
                    self.children.push(ChildNode::Gccodenode(CodeNode::new(generated_code_string)));
                }
            }
            ChildNode::Gcsourcenode(sn) => self.children.push(ChildNode::Gcsourcenode(sn)),
            ChildNode::Gcsourcelistmap(slm) => {
                for child in slm.children {
                    self.children.push(child);
                }
            }
            _ => {}
        }
    }


    pub fn prepend(&mut self, generated_code: ChildNode, source: String, original_source: String) {
        match generated_code {
            ChildNode::Gcstring(generated_code_string) => {
                if source.len() == 0 {
                    self.children
                        .insert(0,
                                ChildNode::Gcsourcenode(SourceNode::new(generated_code_string,
                                                                        original_source,
                                                                        source,
                                                                        1)));
                } else if self.children.len() > 0 {
                    let mut child = self.children.get(self.children.len() - 1).unwrap().clone();
                    match child {
                        ChildNode::Gccodenode(ref mut cn) => {}  // TODO this cn.prepend(&generated_code_string),
                        _ => {
                            self.children
                                .insert(0,
                                        ChildNode::Gccodenode(CodeNode::new(generated_code_string)))
                        }
                    }
                } else {
                    self.children
                        .insert(0,
                                ChildNode::Gccodenode(CodeNode::new(generated_code_string)));
                }
            }
            ChildNode::Gcsourcenode(sn) => self.children.insert(0, ChildNode::Gcsourcenode(sn)),
            ChildNode::Gcsourcelistmap(mut slm) => {
                slm.children.reverse();
                for child in slm.children {
                    self.children.insert(0, child);
                }
            }
            _ => {}
        }
    }

    pub fn map_generated_code(&mut self, f: &Fn(String) -> String) {
        let mut normalized_nodes: Vec<ChildNode> = Vec::new();
        let children = self.children.clone();
        for child in children {
            match child {
                ChildNode::Gccodenode(code_node) => {
                    for new_node in code_node.get_normalized_nodes() {
                        normalized_nodes.push(ChildNode::Gccodenode(new_node));
                    }
                }
                ChildNode::Gcsourcenode(source_node) => {
                    for new_node in source_node.get_normalized_nodes() {
                        normalized_nodes.push(ChildNode::Gcsinglelinenode(new_node));
                    }
                }
                ChildNode::Gcsinglelinenode(single_line_node) => {
                    for new_node in single_line_node.get_normalized_nodes() {
                        normalized_nodes.push(ChildNode::Gcsinglelinenode(new_node));
                    }
                }
                _ => {}
            }
        }

        let mut optimized_nodes: Vec<ChildNode> = Vec::new();

        for nodes in normalized_nodes {
            let sln = match nodes {
                ChildNode::Gccodenode(code_node) => {
                    Some(ChildNode::Gccodenode(code_node.map_generated_code(f)))
                }
                ChildNode::Gcsourcenode(source_node) => {
                    Some(ChildNode::Gcsourcenode(source_node.map_generated_code(f).unwrap()))
                }
                ChildNode::Gcsinglelinenode(single_line_node) => {
                    println!("am in here");
                    Some(ChildNode::Gcsinglelinenode(single_line_node.map_generated_code(f)))
                }
                _ => None,
            };
            // fix it here
            match sln {
                Some(s) => {
                    if optimized_nodes.len() == 0 {
                        optimized_nodes.push(s);
                    } else {
                        let last = optimized_nodes.get(optimized_nodes.len() - 1).unwrap().clone();
                        let merged_node: Option<ChildNode> = match last {
                            ChildNode::Gccodenode(mut code_node) => {
                                match s.clone() {
                                    ChildNode::Gccodenode(cn) => {
                                        Some(ChildNode::Gccodenode(code_node.merge(cn).clone()))
                                    }
                                    _ => None,
                                }
                                // Some(ChildNode::Gccodenode(code_node.merge(sln.unwrap())))
                            }
                            ChildNode::Gcsourcenode(mut source_node) => {
                                match s.clone() {
                                    ChildNode::Gcsourcenode(cn) => {
                                        Some(ChildNode::Gcsourcenode(source_node.merge(cn).clone()))
                                    }
                                    _ => None,
                                }
                            }
                            ChildNode::Gcsinglelinenode(mut single_line_node) => {
                                match s.clone() {
                                    ChildNode::Gcsinglelinenode(cn) => {
                                        let sln_node = single_line_node.merge(cn);
                                        match sln_node.clone() {
                                            Some(sln_sn) => {
                                                Some(ChildNode::Gcsinglelinenode(sln_sn.clone()))
                                            }
                                            _ => None,
                                        }
                                    }
                                    _ => None,
                                }
                            }
                            _ => None,
                        };

                        match merged_node {
                            Some(s) => {
                                let len = optimized_nodes.clone().len() - 1;
                                optimized_nodes.remove(len);
                                optimized_nodes.push(s);
                            }
                            None => {
                                //optimized_nodes.push(s)
                            }
                        }
                    }
                }
                None => {}
            }
        }
        self.new_from_code(Gc::CodeVec(optimized_nodes));
    }

    fn to_string(&self) -> String {
        let mut output: String = String::from("");
        let children = self.children.clone();
        for child in children {
            match child {
                ChildNode::Gcsinglelinenode(sln) => output += sln.get_generated_code(),
                _ => {}
            };
        }
        output
    }
    // options
    pub fn to_string_with_source_map(&mut self, options: String) -> StringToSrc {
        let mut mc: MappingsContext = MappingsContext::new();

        let mut src: String = String::from("");
        for child in &self.children {
            match child {
                &ChildNode::Gcsourcenode(ref sln) => src += sln.get_generated_code(),
                // ChildNode::Gccodenode(sln) => src += sln.get_generated_code(),
                &ChildNode::Gcsinglelinenode(ref sln) => src += sln.get_generated_code(),
                &ChildNode::Gcstring(ref sln) => src += &sln,
                _ => {}
            }
        }


        let mut mappings: String = String::from("");
        for child in &self.children {
            match child {
                &ChildNode::Gcsourcenode(ref sln) => mappings += &sln.get_mappings(&mut mc),
                &ChildNode::Gccodenode(ref sln) => mappings += &sln.get_mappings(&mut mc),
                &ChildNode::Gcsinglelinenode(ref sln) => mappings += &sln.get_mappings(&mut mc),
                _ => {}
            };
        }
        let arrays = mc.get_arrays();
        let map: SrcMap = SrcMap::new(String::from("3"),
                                      options,
                                      arrays.sources,
                                      arrays.sources_content,
                                      mappings);
        StringToSrc::new(src, map)
    }
}

#[derive(Clone)]
pub enum ChildNode {
    Gcstring(String),
    Gccodenode(CodeNode),
    Gcsourcelistmap(SourceListMap),
    Gcsourcenode(SourceNode),
    Gcsinglelinenode(SingleLineNode),
}

pub struct StringToSrc {
    pub source: String,
    pub map: SrcMap,
}

impl StringToSrc {
    pub fn new(sources: String, src_map: SrcMap) -> Self {
        StringToSrc {
            source: sources,
            map: src_map,
        }
    }
}

pub struct SrcMap {
    pub version: String,
    pub file: String,
    pub sources: Vec<String>,
    pub sources_content: Vec<String>,
    pub mappings: String,
}

impl SrcMap {
    pub fn new(version: String,
               file: String,
               sources: Vec<String>,
               sources_content: Vec<String>,
               mappings: String)
               -> Self {
        SrcMap {
            version: version,
            file: file,
            sources: sources,
            sources_content: sources_content,
            mappings: mappings,
        }
    }
}

pub enum Gc {
    Code(String),
    CodeVec(Vec<ChildNode>),
}
