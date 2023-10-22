use crate::{files, imports, rules};
use std::{error::Error, path::Path};

pub fn visit_path(
    root: &Path,
    disallowed_imports: Vec<String>,
    current: &Path,
) -> Result<(), Box<dyn Error>> {
    let map = files::list_files_and_directories(current)?;
    let directories = map.get("directories").unwrap();
    let files = map.get("files").unwrap();

    visit_directories(root, &disallowed_imports, &current, &directories)?;
    check_files_for_disallowed_imports(root, &disallowed_imports, &current, &files)?;

    Ok(())
}

fn check_files_for_disallowed_imports(
    root: &Path,
    disallowed_imports: &Vec<String>,
    current: &Path,
    files: &Vec<String>,
) -> Result<(), Box<dyn Error>> {
    for file in files {
        if !file.ends_with(".ts") {
            continue;
        }
        let full_path = current.join(file);
        let relative_path = full_path.strip_prefix(root)?;
        let imports = imports::read_imports_from_file(&full_path)?;
        for import in imports {
            for disallowed_import in disallowed_imports {
                if import.starts_with(disallowed_import) {
                    println!(
                        "{} \n  imports from {}",
                        relative_path.to_str().expect(""),
                        disallowed_import,
                    );
                }
            }
        }
    }
    Ok(())
}

fn visit_directories(
    root: &Path,
    disallowed_imports: &Vec<String>,
    current: &Path,
    directories: &Vec<String>,
) -> Result<(), Box<dyn Error>> {
    let current_rules = rules::get_dir_rules(current);
    for child in directories {
        let dir_disallowed_imports = rules::get_child_disallowed_imports(
            root,
            current,
            disallowed_imports,
            &current_rules,
            child,
        );
        let next = current.join(child);
        visit_path(root, dir_disallowed_imports, &next)?;
    }
    Ok(())
}
