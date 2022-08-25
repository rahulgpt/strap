use crate::{Cli, ComponentType, Config};
use std::{fs, path::Path, process};

pub fn get_component_type(config: &Config, args: &Cli) -> ComponentType {
    // component_type will default to Functional
    let mut component_type = match &config.component_type {
        Some(s) => match s.as_str() {
            "functional" | "func" => ComponentType::Functional,
            "class" => ComponentType::Class,
            _ => ComponentType::Functional,
        },
        _ => ComponentType::Functional,
    };

    // override component_type if type flags are provided
    if args.class {
        component_type = ComponentType::Class;
    }
    if args.func {
        component_type = ComponentType::Functional;
    }

    component_type
}

pub fn get_template_path(
    config: &Config,
    base_path: &str,
    comp_type: &ComponentType,
) -> Option<String> {
    let path = config
        .template_path
        .clone()
        .unwrap_or_else(|| format!("{base_path}/.templates"));

    match comp_type {
        ComponentType::Functional => {
            if Path::new(&format!("{path}/functional.js")).exists() {
                Some(format!("{path}/functional.js"))
            } else if Path::new(&format!("{path}/functional")).exists() {
                Some(format!("{path}/functional"))
            } else {
                None
            }
        }
        ComponentType::Class => {
            if Path::new(&format!("{path}/class.js")).exists() {
                Some(format!("{path}/class.js"))
            } else if Path::new(&format!("{path}/class")).exists() {
                Some(format!("{path}/class"))
            } else {
                None
            }
        }
    }
}

pub fn warn_and_exit(err: &str) {
    eprintln!("\x1b[33mWarning\x1b[0m: {err}");
    process::exit(1);
}

pub fn error_and_exit(err: &str) {
    eprintln!("\x1b[31mError\x1b[0m: {err}");
    process::exit(1);
}

pub fn write_component_from_template(
    template_path: &str,
    name_and_extension: (&str, &str),
    resolved_path: &str,
    use_template_extension: bool,
) {
    let mut extension = name_and_extension.1.to_owned();

    if Path::new(template_path).is_dir() {
        let resolved_path = &format!("{}/{}", resolved_path, name_and_extension.0);
        fs::create_dir_all(resolved_path).unwrap();

        let iter = fs::read_dir(template_path).unwrap();

        for entry in iter {
            let entry = entry.unwrap();
            let content = fs::read_to_string(entry.path()).unwrap();
            let content = content.replace("_component", &format!("{}", name_and_extension.0));

            if use_template_extension {
                extension = get_file_extension(entry.file_name().to_str().unwrap());
            }

            if entry.file_name().to_str().unwrap().contains("_component") {
                fs::write(
                    format!("{resolved_path}/{}{}", name_and_extension.0, extension),
                    content,
                )
                .unwrap();
            } else {
                fs::write(
                    format!("{}/{}", resolved_path, entry.file_name().to_str().unwrap()),
                    content,
                )
                .unwrap();
            }
        }
    } else {
        let content = fs::read_to_string(template_path).unwrap();
        let content = content.replace("_component", &format!("{}", name_and_extension.0));
        fs::write(
            format!(
                "{resolved_path}/{}{}",
                name_and_extension.0, name_and_extension.1
            ),
            content,
        )
        .unwrap();
    }
}

pub fn write_default_templates(
    templates: [&str; 3],
    name_and_extension: (&str, &str),
    component_type: &ComponentType,
    resolved_path: &str,
) {
    let resolved_path = &format!("{}/{}", resolved_path, name_and_extension.0);
    fs::create_dir_all(resolved_path).unwrap();

    let mut content = templates[0].replace("_component", &format!("{}", name_and_extension.0));
    fs::write(
        format!("{resolved_path}/index{}", name_and_extension.1),
        content,
    )
    .unwrap();

    match component_type {
        ComponentType::Functional => {
            content = templates[1].replace("_component", &format!("{}", name_and_extension.0));
        }
        ComponentType::Class => {
            content = templates[2].replace("_component", &format!("{}", name_and_extension.0));
        }
    }

    fs::write(
        format!(
            "{resolved_path}/{}{}",
            name_and_extension.0, name_and_extension.1
        ),
        content,
    )
    .unwrap();
}

pub fn generate_config() {
    let config = Config {
        base_path: Some("src/components".to_owned()),
        template_path: Some("src/components/.templates".to_owned()),
        component_type: Some("functional".to_owned()),
        typescript: Some(false),
        verbose_output: Some(false),
        force: Some(false),
    };

    fs::write(
        "strap-config.json",
        serde_json::to_string_pretty(&config).unwrap(),
    )
    .unwrap();
}

fn get_file_extension(file_name_with_extension: &str) -> String {
    file_name_with_extension
        .split(".")
        .next()
        .unwrap()
        .to_owned()
}
