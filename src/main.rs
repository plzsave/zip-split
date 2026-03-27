use clap::Parser;

#[derive(Parser)]
#[command(about = "zipファイル内のディレクトリを個別のzipとして取り出す")]
struct Args {
    /// 元のzipファイルパス
    src: String,
    /// 出力先ディレクトリ (デフォルト: カレント)
    #[arg(short, long, default_value = ".")]
    output: String,
}

fn main() -> zip::result::ZipResult<()> {
    let args = Args::parse();
    zip_split::extract_dirs_as_zips(&args.src, &args.output)
}
