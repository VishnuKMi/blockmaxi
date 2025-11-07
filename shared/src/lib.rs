use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub tech_stack: Vec<String>,
    pub github_url: Option<String>,
    pub live_url: Option<String>,
    pub image_url: String,
    pub category: ProjectCategory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectCategory {
    SmartContract,
    DeFi,
    NFT,
    Infrastructure,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub name: String,
    pub level: u8, // 1-100
    pub category: SkillCategory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SkillCategory {
    Blockchain,
    Language,
    Framework,
    Tool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experience {
    pub company: String,
    pub role: String,
    pub duration: String,
    pub description: String,
    pub achievements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactForm {
    pub name: String,
    pub email: String,
    pub message: String,
}
