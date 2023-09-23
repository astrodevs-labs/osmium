use solidhunter_lib::linter::SolidLinter;
use solidhunter_lib::types::Position;
use std::{fs, path::PathBuf};

struct Finding {
    start: Position,
    end: Position,
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
                            line: splitted_line[1].parse::<usize>().unwrap(),
                            character: splitted_line[2].parse::<usize>().unwrap(),
                        },
                        end: Position {
                            line: splitted_line[3].parse::<usize>().unwrap(),
                            character: splitted_line[4].parse::<usize>().unwrap(),
                        },
                        id: splitted_line[0].to_string(),
                    });
                }
            }
        }
    }

    test_linter(&config, &source, &expected_findings);
}

fn test_linter(config: &str, source: &str, expected_findings: &Vec<Finding>) {
    let mut linter: SolidLinter = SolidLinter::new(&String::from(config));

    let result = linter.parse_file(String::from(source));
    match result {
        Ok(diags) => {
            assert_eq!(
                diags.len(),
                expected_findings.len(),
                "Wrong number of findings for {}",
                source
            );
            let mut found = false;

            for (_, diag) in diags.iter().enumerate() {
                for (_, expected_finding) in expected_findings.iter().enumerate() {
                    if (diag.range.start == expected_finding.start)
                        && (diag.range.end == expected_finding.end)
                        && (diag.id == expected_finding.id)
                    {
                        found = true;
                        break;
                    }
                }
            }
            assert_eq!(found, true, "Can't find the diagnostic for {}", source);
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
    ContractNamePascalCase,
    FunctionMaxLines,
    ImportOnTop,
    LineMaxLen,
    MaxStatesCount,
    FunctionNameCamelCase,
    FunctionParamNameCamelCase,
    UseForbiddenName,
    ReasonString,
    NoInlineAssembly,
    FunctionVisibility,
    OneContractPerFile,
    CustomErrors,
    EventNameCamelCase
    ConstNameSnakeCase,
}
