use std::io::{self, Write};

pub fn test() {
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);
    writeln!(handle, "foo: {}", 42);
}
