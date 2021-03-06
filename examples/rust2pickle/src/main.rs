#![warn(rust_2018_idioms, single_use_lifetimes)]

mod pickle;

use std::{
    fs,
    io::{self, BufWriter, Write},
};
use structopt::StructOpt;

type Result<T, E = anyhow::Error> = std::result::Result<T, E>;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    input_path: std::path::PathBuf,
    #[structopt(parse(from_os_str))]
    output_path: Option<std::path::PathBuf>,
}

fn main() -> Result<()> {
    let args = Cli::from_args();

    let code = fs::read_to_string(&args.input_path)?;
    let syntax = syn::parse_file(&code)?;

    let buf = pickle::to_vec(&syntax);
    if let Some(outpath) = args.output_path {
        fs::write(outpath, buf)?;
    } else {
        let writer = io::stdout();
        let mut writer = BufWriter::new(writer.lock());
        writer.write_all(&buf)?;
        writer.flush()?;
    }
    Ok(())
}
