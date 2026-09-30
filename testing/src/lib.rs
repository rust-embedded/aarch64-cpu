use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
    env, fs, io,
    path::Path,
    process::Command,
};

fn targets(on_nightly: bool) -> &'static [&'static str] {
    if on_nightly {
        // targets that need `-Z build-std`
        &["aarch64v8r-unknown-none"]
    } else {
        &["aarch64-unknown-none", "aarch64-unknown-none-softfloat"]
    }
}

/// All programs in the `examples` directories (+) are treated as snapshot tests
///
/// Each example will be executed using the Cargo runner specified in its source
/// file, expected to exit with exit code `0` and its stdout will be compared
/// against a snapshot file for exact equality.
///
/// The snapshot file is located next to the source file, has the same
/// file name (stem) but uses the `.stdout` file extension.
///
/// The runner must be specified with a comment of the form
/// `// runner: runner-in-path some flags`
///
/// By default, each test will be cross compiled to each target in the
/// `targets()` list. To *not* compile, and run, a test to a certain target add
/// a comment of the form: `// ignore: aarch64-unknown-none-softfloat` to the
/// source file
///
/// Set the `TESTING_UPDATE_SNAPSHOTS` environment variable to make the test
/// runner update all the snapshot files. Note that the exit code will still
/// be checked and a non-zero exit code will fail this test.
///
/// Custom RUSTFLAGS can be set on a test basis using the `// rustflags:`
/// directive. The RHS will be passed as is in the RUSTFLAGS env var.
///
/// Cargo features can also be set on a test basis using the `// features:`
/// directive. The RHS will be passed as is to Cargo via the `--features` flag
///
/// (+) Namely `packages/*/examples`. The reason `examples` is used instead of
/// `src/bin` is that `examples` can use dev-dependencies whereas `src/bin`
/// programs can only use regular dependencies
#[test]
fn snapshot_tests() -> io::Result<()> {
    let packages_dir = root().join("packages");

    let mut rs_set = BTreeSet::new();
    let mut stdout_set = BTreeMap::new();
    let update_snapshots = env::var_os("TESTING_UPDATE_SNAPSHOTS").is_some();
    for package_entry in fs::read_dir(&packages_dir)? {
        let package_entry = package_entry?;
        if !package_entry.file_type()?.is_dir() {
            continue;
        }

        let package_path = package_entry.path();
        let package_name = get_stem(&package_path);
        let examples_dir = package_path.join("examples");
        if !examples_dir.exists() {
            continue;
        }

        for entry in fs::read_dir(&examples_dir)? {
            let entry = entry?;
            if !entry.file_type()?.is_file() {
                continue;
            }

            let path = entry.path();
            let extension = path.extension().unwrap_or_default();
            if extension == "stdout" {
                assert!(
                    path.with_extension("rs").exists(),
                    ".stdout file has no corresponding .rs file: {}",
                    path.display()
                );

                let stdout_name = get_stem(&path);
                stdout_set.insert(stdout_name, path);
                continue;
            } else if extension != "rs" {
                continue;
            }

            let example_name = get_stem(&path);
            assert!(
                !rs_set.contains(&example_name),
                "found more than one example with the same name: {example_name}
do not give examples under different Cargo packages the same name as Cargo will
overwrite the output binary in the $CARGO_TARGET_DIR directory"
            );
            rs_set.insert(example_name);

            unit_test(&package_name, &path, update_snapshots);
        }
    }

    if update_snapshots {
        // remove unused `.stdout` files
        for (stdout_name, stdout_path) in stdout_set {
            if rs_set.contains(&stdout_name) {
                continue;
            }

            _ = fs::remove_file(stdout_path);
        }
    } else {
        assert!(
            stdout_set.keys().eq(rs_set.iter()),
            ".rs files and .stdout files do not match in number"
        );
    }

    assert_ne!(0, rs_set.len(), "no tests found");

    Ok(())
}

fn unit_test(package_name: &str, rs_path: &Path, mut update_snapshots: bool) {
    let name = get_stem(&rs_path);
    eprintln!("\nrun {package_name}/{name}");

    let Directives {
        features,
        ignores,
        runner,
        rustflags,
    } = extract_directives(&rs_path);
    let mut expected = None;
    for target in targets(rustversion::cfg!(nightly)) {
        if ignores.contains(*target) {
            continue;
        }
        eprintln!("∟ {target}");

        let runner_key = format!(
            "CARGO_TARGET_{}_RUNNER",
            target.replace("-", "_").to_ascii_uppercase()
        );
        let mut cargo = Command::new("cargo");
        cargo.arg("run");
        if rustversion::cfg!(nightly) {
            cargo.arg("-Zbuild-std=core,compiler_builtins");
        }
        if let Some(rustflags) = &rustflags {
            cargo.env("RUSTFLAGS", rustflags);
        }
        cargo.args(["-p", &package_name, "--example", &name, "--target", target]);
        if let Some(features) = &features {
            cargo.args(["--features", features]);
        }
        eprintln!("Running {:?}", cargo);
        let output = cargo
            .env(runner_key, &runner)
            .current_dir(root())
            .output()
            .unwrap_or_else(|_| panic!("failed to execute Cargo command for test {name}"));

        assert!(
            output.status.success(),
            "Cargo stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        );

        let stdout_path = rs_path.with_extension("stdout");
        let actual = String::from_utf8(output.stdout)
            .unwrap_or_else(|_| panic!("output of test {name} is not valid UTF-8"));
        if update_snapshots {
            // only update on first target; check the rest of targets
            update_snapshots = false;

            fs::write(&stdout_path, actual)
                .unwrap_or_else(|_| panic!("could not write file {}", stdout_path.display()));
        } else {
            let expected = expected.get_or_insert_with(|| {
                fs::read_to_string(&stdout_path)
                    .unwrap_or_else(|_| panic!("could not read file {}", stdout_path.display()))
            });

            pretty_assertions::assert_eq!(*expected, actual);
        }
    }
}

struct Directives {
    features: Option<String>,
    ignores: HashSet<String>,
    runner: String,
    rustflags: Option<String>,
}

fn extract_directives(path: &Path) -> Directives {
    let contents =
        fs::read_to_string(path).unwrap_or_else(|_| panic!("could not read {}", path.display()));

    let mut runner = None;
    let mut features = None;
    let mut ignores = HashSet::new();
    let mut rustflags = None;
    for line in contents.lines() {
        let Some(comment) = line.trim().strip_prefix("//") else {
            continue;
        };

        let comment = comment.trim();
        if let Some(value) = comment.strip_prefix("runner:") {
            assert!(runner.is_none(), "runner specified more than once");
            runner = Some(value.trim().to_string());
        } else if let Some(values) = comment.strip_prefix("ignore:") {
            for item in values.split_whitespace() {
                ignores.insert(item.to_string());
            }
        } else if let Some(flags) = comment.strip_prefix("rustflags:") {
            assert!(rustflags.is_none(), "rustflags specified more than once");
            rustflags = Some(flags.trim().to_string())
        } else if let Some(list) = comment.strip_prefix("features:") {
            assert!(features.is_none(), "features specified more than once");
            features = Some(list.trim().to_string())
        }
    }

    let runner = runner.unwrap_or_else(|| {
        panic!(
            "file {} does not specify a runner with `// runner: my-runner`",
            path.display(),
        )
    });

    for target in &ignores {
        if !targets(false).contains(&target.as_str()) && !targets(true).contains(&target.as_str()) {
            panic!("unsupported target in ignore directive: {target}")
        }
    }

    Directives {
        features,
        ignores,
        runner,
        rustflags,
    }
}

fn get_stem(path: &Path) -> String {
    let stem = path
        .file_stem()
        .unwrap_or_else(|| panic!("could not extract stem from {}", path.display()));
    stem.to_str()
        .unwrap_or_else(|| panic!("file path {} is not valid UTF-8", path.display()))
        .to_string()
}

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .expect("directory layout does not match expectations")
}
