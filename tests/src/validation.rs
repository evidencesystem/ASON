pub fn metaschema() {
    let path = "../schemas/article.schema.json";
    let file = std::fs::File::open(path).expect("Unable to read file");
    let json_object: serde_json::Value =
        serde_json::from_reader(file).expect("file should be proper JSON");

    let schema = serde_json::json!(json_object);

    let validator = jsonschema::validator_for(&schema).expect("Failed to build validator");

    // Validate schema with automatic draft detection
    assert!(validator.is_valid(&schema));
    assert!(validator.validate(&schema).is_ok());

    println!("Schemas successfully validated");
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
