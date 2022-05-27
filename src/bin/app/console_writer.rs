
/// WASMConsoleWriter logs traces to the console.
pub struct WASMConsoleWriter;

impl std::io::Write for WASMConsoleWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize,std::io::Error> {
        gloo::console::log!(String::from_utf8(buf.to_vec()).expect("Valid utf8 from \
            Tracing"));
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), std::io::Error> {
        gloo::console::log!("Writer flushed");
        Ok(())
    }
}
