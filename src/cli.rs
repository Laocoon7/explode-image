use crate::prelude::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short = 'c', long = "columns", value_name = "COLUMNS")]
    pub columns: u32,

    #[arg(short = 'r', long = "rows", value_name = "ROWS")]
    pub rows: u32,

    #[arg(long = "padding", value_name = "PADDING", value_parser = parse_tuple::<u32, u32>)]
    pub padding: Option<(u32, u32)>,

    #[arg(long = "offset", value_name = "OFFSET", value_parser = parse_tuple::<u32, u32>)]
    pub offset: Option<(u32, u32)>,

    #[arg(short = 'i', long = "input", value_name = "PATH/FILE.EXT")]
    pub image_input: PathBuf,

    #[arg(short = 'o', long = "output", value_name = "PATH")]
    pub image_output_folder: Option<PathBuf>,

    #[arg(short = 'f', long = "fmt", value_name = "FORMAT")]
    pub image_ext: Option<OsString>,
}

fn parse_tuple<T, U>(
    s: &str,
) -> std::result::Result<(T, U), Box<dyn std::error::Error + Send + Sync + 'static>>
where
    T: std::str::FromStr,
    T::Err: std::error::Error + Send + Sync + 'static,
    U: std::str::FromStr,
    U::Err: std::error::Error + Send + Sync + 'static,
{
    let pos = s
        .find(", ")
        .ok_or_else(|| format!("invalid x, y: no `, ` found in `{}`", s))?;
    Ok((s[..pos].parse()?, s[pos + 2..].parse()?))
}
