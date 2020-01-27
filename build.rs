fn main() {
    cc::Build::new().file("tests/malloc.c").compile("malloc");
}
