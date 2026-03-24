use serde::Deserialize;

// ── Structs ──────────────────────────────────────────────────────────

#[derive(Clone, Deserialize)]
pub struct EducationEntry {
    pub institution: String,
    pub degree: String,
    pub location: String,
    pub start: String,
    pub end: String,
    pub concentrations: Option<Vec<String>>,
    pub minor: Option<String>,
    pub courses: Option<Vec<String>>,
    pub awards: Option<Vec<String>>,
}

#[derive(Clone, Deserialize)]
pub struct Education {
    pub entry: Vec<EducationEntry>,
}

#[derive(Clone, Deserialize)]
pub struct WorkExperience {
    pub title: String,
    pub organization: String,
    pub location: String,
    pub start: String,
    pub end: String,
    pub bullets: Vec<String>,
}

#[derive(Clone, Deserialize)]
pub struct VolunteerExperience {
    pub title: String,
    pub organization: String,
    pub location: Option<String>,
    pub date: Option<String>,
    pub start: Option<String>,
    pub end: Option<String>,
}

#[derive(Clone, Deserialize)]
pub struct Membership {
    pub role: String,
    pub organization: String,
    pub abbreviation: Option<String>,
    pub start: String,
}

#[derive(Clone, Deserialize)]
pub struct Athletics {
    pub team: String,
    pub location: String,
    pub role: String,
    pub start: String,
    pub end: String,
    pub achievements: Vec<String>,
}

#[derive(Clone, Deserialize)]
pub struct Extracurricular {
    pub title: String,
    pub organization: String,
    pub location: String,
    pub start: String,
    pub end: String,
    pub achievements: Vec<String>,
}

#[derive(Clone, Deserialize)]
pub struct Language {
    pub name: String,
    pub level: String,
}

#[derive(Clone, Deserialize)]
pub struct Interest {
    pub name: String,
}

#[derive(Clone, Deserialize)]
pub struct CvData {
    pub education: Education,
    pub work_experience: Vec<WorkExperience>,
    pub volunteer_experience: Vec<VolunteerExperience>,
    pub membership: Vec<Membership>,
    pub athletics: Vec<Athletics>,
    pub extracurricular: Vec<Extracurricular>,
    pub language: Vec<Language>,
    pub interest: Vec<Interest>,
}

pub fn get_cv() -> CvData {
    let toml_str = include_str!("../../assets/data/curriculum_vitae.toml");
    toml::from_str(toml_str).expect("Failed to parse cv.toml")
}
