use sourceNode::SourceNode;
use code_node::CodeNode;
use source_list_map::sourceListMap;
use mappings_context::mappings_context;

pub fn from_string_with_source_map(&self, code: &str, map: mappings_context) -> sourceListMap {
    let sources = &map.sources;
    let sources_content = &map.sources_content;
    let mappings: Vec<&str> = &map.mappings.split(";").collect();
    let lines: Vec<&str> = code.split("\n").collect();
    let nodes = Vec![];

    let current_line = 1;
    let current_source_idx = 0;
    // TODO check
    let current_source_node_line;
    let currentNode;

    for (idx, mapping) in mappings.iter().enumerate() {
        let line = line[i];
        // TODO line check
        if idx != lines.len() - 1 {
            line += "\n";
        }
        if !mapping {
            add_code(line)
        }
        mapping = {value: 0, rest: mapping};
        let line_added = false;

        while mapping.rest {
            line_added = process_mapping(mapping, line, line_added);
        }
        if !line_added {
            add_code(line);
        }
    }

    if mappings.len() < lines.len() {
        let idx = mappings.len();
        while !lines[idx].trim() && idx < lines.len() - 1 {
            add_code(lines[idx]+"\n");
            idx += 1;
        }
        add_code(lines.slice(idx).join("\n"));
    }

    sourceListMap::new(nodes)
}

fn add_code(generated_code: String) -> &str {
    CodeNode::add_generated_code(generated_code)
}
