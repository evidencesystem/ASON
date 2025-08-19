fn inline_refs(mut schema: serde_json::Value, base_path: &std::path::Path) -> serde_json::Value {
    let mut defs = serde_json::json!({});
    let mut seen = std::collections::HashMap::new();
    flatten_refs(&mut schema, base_path, &mut defs, &mut seen);
    if !defs.as_object().unwrap().is_empty() {
        schema
            .as_object_mut()
            .unwrap()
            .insert("$defs".to_string(), defs);
    }
    schema
}

fn flatten_refs(
    schema: &mut serde_json::Value,
    base_path: &std::path::Path,
    defs: &mut serde_json::Value,
    seen: &mut std::collections::HashMap<std::path::PathBuf, String>,
) {
    match schema {
        serde_json::Value::Object(map) => {
            if let Some(serde_json::Value::String(ref_path)) = map.get("$ref") {
                if !ref_path.starts_with("./") {
                    return;
                }
                let ref_file = base_path.join(ref_path);

                if !seen.contains_key(&ref_file) {
                    let name = ref_file.file_stem().unwrap().to_string_lossy().to_string();
                    seen.insert(ref_file.clone(), name.clone());

                    let content = std::fs::read_to_string(&ref_file).unwrap();
                    let mut parsed: serde_json::Value = serde_json::from_str(&content).unwrap();

                    flatten_refs(&mut parsed, ref_file.parent().unwrap(), defs, seen);
                    defs.as_object_mut().unwrap().insert(name.clone(), parsed);
                }

                map.insert(
                    "$ref".to_string(),
                    serde_json::Value::String(format!("#/$defs/{}", seen[&ref_file])),
                );
            }
            for v in map.iter_mut() {
                flatten_refs(v.1, base_path, defs, seen);
            }
        }
        serde_json::Value::Array(arr) => {
            for v in arr.iter_mut() {
                flatten_refs(v, base_path, defs, seen);
            }
        }
        _ => {}
    }
}

pub fn flatten() {
    let path = std::path::Path::new("../schemas//nodes/ason.json");
    let content = std::fs::read_to_string(path).unwrap();
    let schema: serde_json::Value = serde_json::from_str(&content).unwrap();

    let flattened = inline_refs(schema, path.parent().unwrap());

    let file = std::fs::File::create("../schemas//asonSchema.json").unwrap();
    let _ = serde_json::to_writer_pretty(file, &flattened);

    // println!("{}", serde_json::to_string_pretty(&flattened).unwrap());
}
