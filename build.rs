use std::path::Path;
use syntect::parsing::SyntaxSetBuilder;
use syntect::dumps::dump_to_file;

fn main() {
    // Define the path to the `packdump` file.
    let packdump_path = "syntaxes.packdump";

    // Check if the `packdump` file already exists.
    if Path::new(packdump_path).exists() {
        println!("Syntax packdump already exists. Skipping compilation.");
    } else {
        println!("cargo:warning=Building syntax set...");

        // Build the syntax set from the syntaxes folder.
        let mut builder = SyntaxSetBuilder::new();
        builder.add_from_folder("bat-syntaxes/assets/syntaxes", true).expect("Failed to add syntaxes");
        let syntax_set = builder.build();

        // Dump the compiled syntax set to the specified file.
        dump_to_file(&syntax_set, packdump_path).expect("Failed to dump syntax set");
    }

    // Re-run the build script if any syntax file changes.
    println!("cargo:rerun-if-changed=bat-syntaxes/assets/syntaxes");
}
