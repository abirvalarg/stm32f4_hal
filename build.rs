fn main() {
    cc::Build::new()
        .file("src/interrupt.c")
        .file("src/malloc.c")
        .file("src/volatile.c")
        .compile("c-part.a");
}
