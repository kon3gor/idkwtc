use std::{
    fs::File,
    io::{Read, Write},
};

use serde::{Deserialize, Serialize};
use tera::Tera;

#[derive(Serialize, Deserialize)]
struct ProjectsHolder {
    projects: Vec<Project>,
}

#[derive(Serialize, Deserialize)]
struct Project {
    name: String,
    description: String,
    res: Option<Vec<Resource>>,
    mine: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Resource {
    title: Option<String>,
    link: String,
}

fn main() {
    // Create a new Tera instance and add a template from a string
    let mut tera = Tera::default();
    tera.add_template_file("template.html", None).unwrap();

    // Prepare the context with some data
    let mut context = tera::Context::new();
    let mut pf = File::open("projects.toml").unwrap();
    let mut content = String::new();
    pf.read_to_string(&mut content).unwrap();
    let projects: ProjectsHolder = toml::from_str(content.as_str()).unwrap();
    context.insert("projects", &projects.projects);

    // Render the template with the given context
    let rendered = tera.render("template.html", &context).unwrap();
    std::fs::create_dir("dest").unwrap();
    let mut r = File::create("dest/index.html").unwrap();
    r.write_all(rendered.as_bytes()).unwrap();
}
