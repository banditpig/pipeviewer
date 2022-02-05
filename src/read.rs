use crate::CHUNK_SIZE;
// use crossbeam_channel::{bounded, unbounded, Receiver, Sender};
use crossbeam_channel::Sender;
use std::fs::File;
use std::io::{self, BufReader, Read, Result};

pub fn read_loop(infile: &str, stats_tx: Sender<usize>, write_tx: Sender<Vec<u8>>) -> Result<()> {
    let mut reader = create_reader(infile);

    let mut buffer = [0; CHUNK_SIZE];
    loop {
        let num_read = match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };

        let _ = stats_tx.send(num_read);
        if write_tx.send(Vec::from(&buffer[..num_read])).is_err() {
            break;
        }
    }

    let _ = stats_tx.send(0);
    let _ = write_tx.send(Vec::new());

    Ok(())
}

fn create_reader(infile: &str) -> Box<dyn Read> {
    let reader: Box<dyn Read> = if !infile.is_empty() {
        Box::new(BufReader::new(File::open(infile).unwrap()))
    } else {
        Box::new(io::stdin())
    };
    reader
}
