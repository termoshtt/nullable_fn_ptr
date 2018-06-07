
extern crate cc;

fn main() {
    cc::Build::new()
        .file("callback.c")
        .compile("ext");
}
