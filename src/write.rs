use std::fs::File;
use std::io;

use std::io::{BufWriter, ErrorKind, Result, Write};
use std::sync::mpsc::Receiver;

pub fn write_loop(outfile: &str, write_rx: Receiver<Vec<u8>>) -> Result<()> {
    let mut writer = create_writer(outfile);
    loop {
        let buffer = write_rx.recv().unwrap();
        if buffer.is_empty() {
            break;
        }

        if let Err(e) = writer.write_all(&buffer) {
            if e.kind() == ErrorKind::BrokenPipe {
                //terminate cleanly
                return Ok(());
            }
            return Err(e);
        }
    }

    Ok(())
}

fn create_writer(outfile: &str) -> Box<dyn Write> {
    let writer: Box<dyn Write> = if !outfile.is_empty() {
        Box::new(BufWriter::new(File::create(outfile).unwrap()))
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };
    writer
}
