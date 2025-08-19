mod transformation;
mod validation;

fn main() {
    transformation::flatten();
    validation::metaschema();
    // validation::schema();
}
