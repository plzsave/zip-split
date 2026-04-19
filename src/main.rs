use clap::Parser;
use std::path::Path;

#[derive(Parser)]
#[command(about = "zipファイル内のディレクトリを個別のzipとして取り出す")]
struct Args {
    /// 元のzipファイルパス
    src: String,
    /// 出力先ディレクトリ (デフォルト: zipファイルと同名のフォルダ)
    #[arg(short, long)]
    output: Option<String>,
}

fn main() -> zip::result::ZipResult<()> {
    let args = Args::parse();
    let output = args.output.unwrap_or_else(|| {
        Path::new(&args.src)
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string()
    });
    zip_split::extract_dirs_as_zips(&args.src, &output)
}
