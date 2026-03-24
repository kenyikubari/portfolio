#[derive(Clone)]
pub struct Project {
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub link: Option<String>,
    pub repo: Option<String>,
}

pub fn get_projects() -> Vec<Project> {
    vec![
        Project {
            title: "Bachelor's Thesis: Streamwise Aspect Ratios & Flow Separation".into(),
            description:
                "Experimental study examining the effects of streamwise aspect ratio on flow separation of trapezoidal prisms using particle image velocimetry (PIV). Analyzed six aspect ratio cases to investigate mean flow characteristics, Reynolds stresses, and turbulent kinetic energy distributions.".into(),
            tags: vec!["PIV".into(), "Fluid Dynamics".into(), "Turbulence".into(), "CFD".into()],
            link: None,
            repo: None,
        },
        Project {
            title: "Capstone: Rear Service Door Design (MCI)".into(),
            description:
                "Developed a systematic door design process for Motor Coach Industries addressing
                water intrusion, latching, and binding issues. Final design used Carbon Fiber Epoxy 
                and 6061-T6 Aluminum with truss geometry — 80% reduction in max stress and 21.53 mm less displacement under three-point bending.".into(),
            tags: vec!["Mechanical Design".into(), "FEA".into(), "Carbon Fibre".into(), "AISI 1020".into()],
            link: None,
            repo: None,
        },
    ]
}