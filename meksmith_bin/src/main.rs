use clap::Parser;

#[derive(Parser, Debug)]
#[command(arg_required_else_help = true)]
struct CliArgs {
    #[arg(short, long)]
    input_file_path: String,
}

fn main() {
    let cli_args = CliArgs::parse();
    println!("{:?}", cli_args);
}
