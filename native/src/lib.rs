#[macro_use]
extern crate neon;

use neon::vm::{Call, JsResult};
use neon::js::{JsArray, JsBoolean, JsObject, JsString, Object};
use neon::mem::Handle;
mod base64_vlq;
mod code_node;
mod helper;
mod mappings_context;
mod source_node;
mod single_line_node;
mod source_list_map;

use source_list_map::{SourceListMap, ChildNode};



fn hello(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    Ok(JsString::new(scope, "hello node").unwrap())
}


fn should_map_code_with_many_lines_in_time(call: Call) -> JsResult<JsObject> {
    let scope = call.scope;
    let options = try!(try!(call.arguments.require(scope, 0)).check::<JsObject>());
    let js_object: Handle<JsObject> = JsObject::new(scope);
    let file: String = try!(options.get(scope, "file")).check::<JsString>().unwrap().value();
    let mut output = String::with_capacity(160010);

    for i in 0..200000 {
        output.push_str("MyLine\n");
    }
    output.push_str("MyLine\n");

    let mut slm = SourceListMap::new(String::from(""), String::from(""), String::from(""), None);
    slm.add(ChildNode::Gcstring(output.clone()),
            file.clone(),
            output.clone());
    slm.map_generated_code(&times2);
    let out = slm.to_string_with_source_map(file);
    println!("out  {}", out.source);
    try!(js_object.set("source", JsString::new(scope, &out.source).unwrap()));
    try!(js_object.set("fromSource", JsString::new(scope, &output.clone()).unwrap()));

    let map: Handle<JsObject> = JsObject::new(scope);
    try!(map.set("version", JsString::new(scope, &out.map.version).unwrap()));
    try!(map.set("file", JsString::new(scope, &out.map.file).unwrap()));
    let len = out.map.sources.len();
    let source_array = JsArray::new(scope, len as u32);

    for i in 0..len {
        try!(source_array.set(i as u32,
                              JsString::new(scope, out.map.sources.get(i).unwrap()).unwrap()));
    }
    try!(map.set("sources", source_array));
    try!(map.set("mappings", JsString::new(scope, &out.map.mappings).unwrap()));
    try!(js_object.set("map", map));

    Ok(js_object)
}

fn times2(line: String) -> String {
    line
}

register_module!(m, {
    m.export("hello", hello)?;
    m.export("should_map_code_with_many_lines_in_time",
                should_map_code_with_many_lines_in_time)?;
    Ok(())
});
