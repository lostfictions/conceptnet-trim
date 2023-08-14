use csv::ReaderBuilder;
use serde::ser::{SerializeSeq, Serializer};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};

#[derive(Debug, Deserialize)]
struct InRecord {
    _uri: String,
    rel: String,
    start: String,
    end: String,
    data: String,
}

#[derive(Debug, Serialize)]
struct OutRecord {
    rel: String,
    start: String,
    end: String,
    weight: Option<f64>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .quoting(false)
        .from_path("data/assertions.csv")?;

    let file = File::create("data/trimmed.json")?;
    let mut writer = BufWriter::new(file);

    let mut ser = serde_json::Serializer::pretty(&mut writer);

    let mut read_rows = 0u64;
    let mut write_rows = 0u64;

    let mut seq = ser.serialize_seq(None)?;
    for result in rdr.deserialize() {
        let in_rec: InRecord = result?;
        read_rows += 1;
        if read_rows % 1_000_000 == 0 {
            println!("row {:?}", read_rows);
        }

        if in_rec.start.starts_with("/c/en") && in_rec.end.starts_with("/c/en") {
            let data: Value = serde_json::from_str(&in_rec.data)?;

            let out_rec = OutRecord {
                rel: in_rec.rel,
                start: in_rec.start,
                end: in_rec.end,
                weight: data["weight"].as_f64(),
            };

            seq.serialize_element(&out_rec)?;
            write_rows += 1;
        }
    }
    seq.end()?;
    writer.flush()?;
    println!("read {:?} rows, wrote {:?} rows", read_rows, write_rows);

    Ok(())
}
