use clap::Parser;
use solidhunter_lib::linter::SolidLinter;
use solidhunter_lib::rules::rule_impl::create_rules_file;
use solidhunter_lib::types::LintResult;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(
        short = 'p',
        long = "path",
        default_value = ".",
        help = "Specify project path"
    )]
    project_path: Vec<String>,

    #[arg(
        short = 'f',
        long = "file",
        default_value = "",
        help = "Specify a single file to lint"
    )]
    file_to_lint: String,

    #[arg(
        short = 'e',
        long = "exclude",
        help = "Exclude part of the project path"
    )]
    ignore_path: Vec<String>,

    #[arg(
        short = 'r',
        long = "rules",
        default_value = ".solidhunter.json",
        help = "Specify rules file"
    )]
    rules_file: String,

    #[arg(
        short = 'j',
        long = "json_output",
        default_value = "false",
        help = "Outputs a json format instead"
    )]
    to_json: bool,

    #[arg(
        short = 'v',
        long = "verbose",
        default_value = "false",
        help = "Verbose output"
    )]
    verbose: bool,

    #[arg(
        short = 'i',
        long = "init",
        default_value = "false",
        help = "Initialize rules file"
    )]
    init: bool,
}

fn lint_folder(args: Args) {
    let mut linter: SolidLinter = SolidLinter::new(&args.rules_file);
    let mut result = Vec::new();
    for path in args.project_path {
        result.append(&mut linter.parse_folder(path));
    }
    for res in result {
        print_result(res);
    }
}

fn print_result(result: LintResult) {
    match result {
        Ok(diags) => {
            for diag in diags {
                println!("{}", &diag);
            }
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}

fn main() {
    let args = Args::parse();

    if !args.to_json {
        println!();
        println!("SolidHunter: Fast and efficient Solidity linter");
        println!(
            "By {} - v{} - GNU GPL v3",
            env!("CARGO_PKG_AUTHORS"),
            env!("CARGO_PKG_VERSION")
        );
        println!();
    }

    if args.verbose {
        println!("Verbose output enabled");
        println!("Project path: {:?}", args.project_path);
        println!("Exclude path: {:?}", args.ignore_path);
        println!("Using rules file: {}", args.rules_file);
        println!("Verbose output: {}", args.verbose);
    }

    if args.init {
        println!("Initializing rules file...");
        create_rules_file(".solidhunter.json");
        println!("Done!");
        return;
    }

    if !args.to_json && args.file_to_lint.is_empty() {
        lint_folder(args);
    } else if !args.file_to_lint.is_empty() {
        let mut linter: SolidLinter = SolidLinter::new(&args.rules_file);

        let result = linter.parse_file(args.file_to_lint);
        if !args.to_json {
            print_result(result);
        } else {
            match result {
                Ok(diags) => {
                    let json = serde_json::to_string_pretty(&diags);
                    match json {
                        Ok(j) => {
                            println!("{}", j);
                        }
                        Err(e) => {
                            println!("{}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
    }
}
