use std::fs;
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
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir_path = temp_dir.path();

    // Copy the markdown file to the temporary directory
    let md_file_name = md_file.file_name().unwrap();
    let temp_md_path = temp_dir_path.join(md_file_name);
    fs::copy(md_file, &temp_md_path).unwrap();

    let template_string: String = format!(
        "#import \"@preview/cmarker:0.1.8\"\n#import \"@preview/mitex:0.2.6\": mitex\n\n// 正文使用宋体，视觉更清秀，适合简历\n#set text(font: (\"Source Han Serif SC\", \"Noto Serif CJK SC\", \"Songti SC\", \"SimSun\", \"Times New Roman\", \"serif\"), size: 11pt, weight: \"regular\", fill: rgb(\"#333333\"))\n#set par(justify: true, leading: 0.65em)\n\n// 标题使用黑体，更加醒目\n#show heading: set block(above: 1.5em, below: 1em)\n#show heading: set text(font: (\"PingFang SC\", \"Microsoft YaHei\", \"Source Han Sans SC\", \"Arial\", \"sans-serif\"), weight: \"bold\", fill: black)\n\n#show link: set text(fill: blue)\n#show raw.where(block: true): set block(fill: luma(240), inset: 10pt, radius: 4pt, width: 100%)\n#show raw: set text(font: (\"Consolas\", \"Monaco\", \"Courier New\", \"monospace\"))\n\n#cmarker.render(\nread(\"{}\"),\nscope: (image: (path, alt: none) => image(path, alt: alt)),\nmath: mitex\n)",
        md_file_name.to_str().unwrap()
    );

    let temp_typ_file_name = "temp.typ";
    let temp_typ_path = temp_dir_path.join(temp_typ_file_name);
    let mut temp_typ_file = fs::File::create(&temp_typ_path).unwrap();
    temp_typ_file.write_all(template_string.as_bytes()).unwrap();

    let output_file_name = Path::new(output).file_name().unwrap();
    let temp_output_path = temp_dir_path.join(output_file_name);

    cmd!("typst", "compile", "--root", temp_dir_path, &temp_typ_path, &temp_output_path)
        .run()
        .unwrap();

    fs::rename(&temp_output_path, output).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_convert_to_pdf() {
        let temp_dir = tempfile::tempdir().unwrap();
        let md_path = temp_dir.path().join("test.md");
        fs::write(&md_path, "# Test Markdown\nThis is a simple test.").unwrap();

        let output_path = temp_dir.path().join("test.pdf");
        convert_to_pdf(&md_path, output_path.to_str().unwrap());

        assert!(output_path.exists());
    }
}
