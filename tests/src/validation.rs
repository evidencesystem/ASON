pub fn metaschema() {
    let path = std::path::Path::new("../schemas/v0/ason.json");
    let content = std::fs::read_to_string(path).unwrap();
    let schema: serde_json::Value = serde_json::from_str(&content).unwrap();

    // let validator = jsonschema::validator_for(&schema).expect("Failed to build validator");

    // let meta_schema_json: serde_json::Value =
    //     reqwest::blocking::get("https://json-schema.org/draft/2020-12/schema")?.json()?;
    // let compiled = jsonschema::compile(&meta_schema_json)?;

    match jsonschema::meta::validate(&schema) {
        Ok(_) => println!("The schema is valid!"),
        Err(error) => {
            println!("Validation error at {}: {}", error.instance_path, error);
        }
    }
}

#[allow(dead_code)]
pub fn schema() {
    let path = "../schemas/article.schema.json";
    let file = std::fs::File::open(path).expect("Unable to read file");
    let json_object: serde_json::Value =
        serde_json::from_reader(file).expect("file should be proper JSON");
    let schema = serde_json::json!(json_object);

    let folder_path = "../schemas/examples";
    let entries = std::fs::read_dir(folder_path).expect("Failed to read directory");

    for entry in entries {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();

        // Filter only `.json` files
        if path.extension().map_or(false, |ext| ext == "json") {
            let file = std::fs::File::open(&path).expect("Unable to read file");

            // Optional: Read the file
            let json_object: serde_json::Value =
                serde_json::from_reader(file).expect("file should be proper JSON");
            let instance = serde_json::json!(json_object);

            match jsonschema::validate(&schema, &instance) {
                Ok(_) => {
                    let s: &str = path.to_str().expect("Path is not valid UTF-8");
                    println!("The schema {} is valid!", s)
                }
                Err(error) => {
                    println!("Validation error at {}: {}", error.instance_path, error);
                }
            }
        }
    }

    println!("All tests passed!");
}

// pub fn schema() {
//     let schema = json!({"maxLength": 5});
//     let instance = json!("foo");

//     // One-off validation
//     assert!(jsonschema::is_valid(&schema, &instance));
//     assert!(jsonschema::validate(&schema, &instance).is_ok());

//     // Build & reuse (faster)
//     let validator = jsonschema::validator_for(&schema)?;

//     // Fail on first error
//     assert!(validator.validate(&instance).is_ok());

//     // Iterate over errors
//     for error in validator.iter_errors(&instance) {
//         eprintln!("Error: {error}");
//         eprintln!("Location: {}", error.instance_path);
//     }

//     // Boolean result
//     assert!(validator.is_valid(&instance));

//     Result::Ok(())
// }
