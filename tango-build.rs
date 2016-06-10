extern crate tango;
extern crate pandoc;
fn main() {
    tango::process_root().unwrap();
    run_pandoc();
}

fn run_pandoc() {
    let mut pandoc = pandoc::new();
    pandoc.add_input("src/slides.md");
    pandoc.set_output("slides.html");
    pandoc.set_output_format(pandoc::OutputFormat::Revealjs);
    pandoc.add_option(pandoc::PandocOption::Standalone);
    pandoc.set_variable("theme", "mozilla-sandstone");
    pandoc.execute().unwrap();
}
