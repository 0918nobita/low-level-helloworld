extern crate cc;

fn main() {
  cc::Build::new()
    .flag("-c")
    .file("src/syscall.s")
    .compile("libsyscall.a");
}
