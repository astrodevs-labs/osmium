use solidhunter_lib::linter::SolidLinter;
use solidhunter_lib::types::Position;
use std::{fs, path::PathBuf};

struct Finding {
    start: Position,
    length: u64,
    id: String,
}

fn test_directory(base_name: &str) {
    let mut source = String::new();
    let mut config = String::new();
    let mut expected_findings: Vec<Finding> = Vec::new();

    for path in fs::read_dir(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("testdata")
            .join(base_name),
    )
    .unwrap()
    {
        let path = path.unwrap().path();

        if let Some(filename) = path.file_name().and_then(|name| name.to_str()) {
            if filename == "file.sol" {
                source = path.to_str().unwrap().to_string();
            } else if filename == ".solidhunter.json" {
                config = path.to_str().unwrap().to_string();
            } else if filename == "findings.csv" {
                for line in fs::read_to_string(path).unwrap().lines() {
                    let splitted_line: Vec<&str> = line.split(':').collect();
                    expected_findings.push(Finding {
                        start: Position {
                            line: splitted_line[1].parse::<u64>().unwrap(),
                            character: splitted_line[2].parse::<u64>().unwrap(),
                        },
                        length: splitted_line[3].parse::<u64>().unwrap(),
                        id: splitted_line[0].to_string(),
                    });
                }
            }
        }
    }

    test_linter(&config, &source, &expected_findings);
}

fn test_linter(config: &str, source: &str, expected_findings: &Vec<Finding>) {
    println!("{}", config);
    let mut linter: SolidLinter = SolidLinter::new(&String::from(config));

    let result = linter.parse_file(String::from(source));
    match result {
        Ok(diags) => {
            assert_eq!(diags.len(), expected_findings.len());
            for (i, el) in diags.iter().enumerate() {
                assert_eq!(el.id, expected_findings[i].id);
                assert_eq!(el.range.start, expected_findings[i].start);
                assert_eq!(el.range.length, expected_findings[i].length);
            }
        }
        Err(e) => {
            panic!("{}", e);
        }
    }
}

macro_rules! test_directories {
    ($($dir:ident),+ $(,)?) => {$(
        #[allow(non_snake_case)]
        #[test]
        fn $dir() {
            test_directory(stringify!($dir));
        }
    )+};
}

test_directories! {
    LineMaxLen
}
