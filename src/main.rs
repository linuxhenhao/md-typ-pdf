use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use clap::Parser;
use duct::cmd;

#[derive(Debug, Clone, clap::Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the input Markdown file.
    input: String,
    /// Path to the output PDF file.
    output: Option<String>,
}

fn main() {
    println!("Starting Markdown to PDF Converter.");
    let args = Args::parse();
    let output_path = args
        .output
        .clone()
        .unwrap_or_else(|| args.input.clone().replace(".md", ".pdf"));
    convert_to_pdf(Path::new(&args.input), &output_path);
    println!("Done.");
}

/// Converts a Markdown file to PDF using typst-cli.
fn convert_to_pdf(md_file: &Path, output: &str) {
    let template_string: String = "#import \"@preview/cmarker:0.1.7\"\n#import \"@preview/mitex:0.2.6\": mitex\n#cmarker.render(\nread(\"REPLACE_ME\"),\nscope: (image: (path, alt: none) => image(path, alt: alt)),\nmath: mitex\n)"
        .replace("REPLACE_ME", md_file.to_str().unwrap());

    let temp_path = Path::new("temp.typ");
    let mut temp_file = File::create(temp_path).unwrap();
    temp_file.write_all(template_string.as_bytes()).unwrap();

    let output_path = Path::new(output);

    cmd!("typst", "compile", "--root", ".", temp_path, output_path)
        .run()
        .unwrap();

    fs::remove_file(temp_path).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_convert_to_pdf() {
        let md_path = "test.md";
        fs::write(md_path, "# Test Markdown\nThis is a simple test.").unwrap();

        let output = "test.pdf";
        convert_to_pdf(Path::new(md_path), output);

        assert!(Path::new(output).exists());

        fs::remove_file(md_path).unwrap();
        fs::remove_file(output).unwrap();
    }
}
