use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Project {
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub link: Option<String>,
    pub repo: Option<String>,
}

#[derive(Deserialize)]
struct ProjectsFile {
    project: Vec<Project>,
}

pub fn get_projects() -> Vec<Project> {
    let toml_str = include_str!("../../assets/data/projects.toml");
    let file: ProjectsFile = toml::from_str(toml_str).expect("Failed to parse projects.toml");
    file.project
}