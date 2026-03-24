use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Skill {
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
}

#[derive(Deserialize)]
struct SkillsFile {
    skill: Vec<Skill>,
}

pub fn get_skills() -> Vec<Skill> {
    let toml_str = include_str!("../../assets/data/skills.toml");
    let file: SkillsFile = toml::from_str(toml_str).expect("Failed to parse skills.toml");
    file.skill
}