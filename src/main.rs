use clap::{ArgGroup, Parser};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

mod utility;

use utility::{
    error_and_exit, generate_config, get_component_type, get_template_path, warn_and_exit,
    write_component_from_template, write_default_templates,
};

/// Bootstrap react component template.
#[derive(Parser)]
#[clap(
    disable_colored_help = true,
    version,
    override_usage = "strap [OPTIONS] <NAME>"
)]
#[clap(group(
    ArgGroup::new("type")
        .required(false)
        .args(&["func", "class"]),
))]
pub struct Cli {
    /// Component Name
    #[clap(required_unless_present("init"))]
    pub name: Option<String>,

    /// Generate config file
    #[clap(long)]
    pub init: bool,

    /// Generate functional template
    #[clap(long, short)]
    pub func: bool,

    /// Generate class template
    #[clap(long, short)]
    pub class: bool,

    /// Print verbose output
    #[clap(long, short)]
    pub verbose: bool,

    /// Generate typescript files
    #[clap(long, short)]
    pub typescript: bool,

    /// Overwrite existing component
    #[clap(long)]
    pub force: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(rename = "basePath")]
    pub base_path: Option<String>,

    #[serde(rename = "templatePath")]
    pub template_path: Option<String>,

    #[serde(rename = "componentType")]
    pub component_type: Option<String>,

    #[serde(rename = "verboseOutput")]
    pub verbose_output: Option<bool>,

    pub typescript: Option<bool>,

    pub force: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ComponentType {
    Functional,
    Class,
}

fn main() {
    let args = Cli::parse();

    if args.init {
        generate_config();
        println!("🚀 Generated \"strap-config.json\"");
        return;
    }

    let config: Option<String> = match fs::read_to_string("strap-config.json") {
        Ok(s) => Some(s),
        _ => None,
    };

    let config: Config = serde_json::from_str(&config.unwrap_or_else(|| "{}".to_owned())).unwrap();

    let mut base_path = config
        .base_path
        .clone()
        .unwrap_or_else(|| "src/components".to_owned());

    let name = args.name.clone();
    let name = name.unwrap();

    // break name into component name and path
    let mut res: Vec<&str> = name.split("/").collect();
    let component_name = res.pop().unwrap();
    let component_path = res.join("/");

    // determine extension
    let extension = if args.typescript || config.typescript.unwrap_or_default() {
        ".ts"
    } else {
        ".js"
    };

    let default_functional_template = include_str!("./templates/javascript/functional.js");
    let default_class_template = include_str!("./templates/javascript/class.js");
    let default_index_template = format!("export {{ default }} from \"./{}\";", component_name);

    let default_templates: [&str; 3] = [
        &default_index_template,
        default_functional_template,
        default_class_template,
    ];

    // check if the current directory is a node package
    if !Path::new("package.json").exists() {
        error_and_exit("Invoke strap from your project's root directory");
    }

    fs::create_dir_all(&base_path).unwrap();

    let component_type = get_component_type(&config, &args);

    // final resolved path concatenated with the
    // nested component path, if provided
    let resolved_path = base_path.clone() + "/" + &component_path;

    // omit extension
    let component_name: Vec<&str> = component_name.split(".").collect();
    let component_name = component_name[0];

    // Check if the component already exists
    if config.force.unwrap_or_default() || args.force {
    } else {
        if Path::new(&format!("{resolved_path}/{component_name}")).exists() {
            warn_and_exit(&format!(
                "A component with name \"{component_name}\" already exists"
            ));
        }
    }

    // create missing directories
    if component_path.len() != 0 {
        fs::create_dir_all(&format!("{}", resolved_path)).unwrap();
    }

    // resolved template path
    let template_path = get_template_path(&config, &base_path, &component_type);

    match template_path {
        Some(path) => {
            write_component_from_template(&path, (component_name, extension), &resolved_path, false)
        }
        // fallback to default templates if templates are not provided
        None => write_default_templates(
            default_templates,
            (component_name, extension),
            &component_type,
            &resolved_path,
        ),
    }

    // empty string will yield perm errors
    if base_path.len() == 0 {
        base_path = ".".to_owned();
    }

    let component_path = if component_path.len() != 0 {
        "/".to_owned() + &component_path
    } else {
        component_path
    };

    if config.verbose_output.unwrap_or_default() || args.verbose {
        println!("Base Path: \"{}\"", &base_path);

        let template_path = get_template_path(&config, &base_path, &component_type);
        match template_path {
            Some(s) => println!("Template Path: \"{}\"", &s),
            None => println!("Templates: Not found (Using default templates)"),
        }

        println!("Component Type: {component_type:?}\n");
    }

    println!(
        "🚀 Generated \x1b[36m{}\x1b[0m in \x1b[32m\"{}{}\"\x1b[0m",
        component_name, base_path, component_path
    );
}
