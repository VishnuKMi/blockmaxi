use axum::{
    Router,
    extract::State,
    http::{Method, StatusCode},
    response::Json,
    routing::{get, post},
};
use shared::{ContactForm, Experience, Project, ProjectCategory, Skill, SkillCategory};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
struct AppState {
    projects: Arc<Vec<Project>>,
    skills: Arc<Vec<Skill>>,
    experiences: Arc<Vec<Experience>>,
}

#[tokio::main]
async fn main() {
    // Initialize mock data (replace with database in production)
    let state = AppState {
        projects: Arc::new(get_mock_projects()),
        skills: Arc::new(get_mock_skills()),
        experiences: Arc::new(get_mock_experiences()),
    };

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let app = Router::new()
        .route("/api/projects", get(get_projects))
        .route("/api/skills", get(get_skills))
        .route("/api/experience", get(get_experience))
        .route("/api/contact", post(submit_contact))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("🚀 Backend running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn get_projects(State(state): State<AppState>) -> Json<Vec<Project>> {
    Json((*state.projects).clone())
}

async fn get_skills(State(state): State<AppState>) -> Json<Vec<Skill>> {
    Json((*state.skills).clone())
}

async fn get_experience(State(state): State<AppState>) -> Json<Vec<Experience>> {
    Json((*state.experiences).clone())
}

async fn submit_contact(Json(form): Json<ContactForm>) -> StatusCode {
    // In production, save to database or send email
    println!("📧 Contact form: {:?}", form);
    StatusCode::OK
}

// Mock data functions
fn get_mock_projects() -> Vec<Project> {
    vec![
        Project {
            id: 1,
            title: "DeFi Lending Protocol".to_string(),
            description: "Decentralized lending platform with automated liquidation engine and flash loan capabilities".to_string(),
            tech_stack: vec!["Solidity".to_string(), "Hardhat".to_string(), "React".to_string()],
            github_url: Some("https://github.com/yourusername/defi-protocol".to_string()),
            live_url: Some("https://protocol.example.com".to_string()),
            image_url: "/images/defi.jpg".to_string(),
            category: ProjectCategory::DeFi,
        },
        Project {
            id: 2,
            title: "NFT Marketplace".to_string(),
            description: "Full-featured NFT marketplace with royalty distribution and lazy minting".to_string(),
            tech_stack: vec!["Rust".to_string(), "Solana".to_string(), "Anchor".to_string()],
            github_url: Some("https://github.com/yourusername/nft-marketplace".to_string()),
            live_url: None,
            image_url: "/images/nft.jpg".to_string(),
            category: ProjectCategory::NFT,
        },
    ]
}

fn get_mock_skills() -> Vec<Skill> {
    vec![
        Skill {
            name: "Solidity".to_string(),
            level: 95,
            category: SkillCategory::Language,
        },
        Skill {
            name: "Rust".to_string(),
            level: 90,
            category: SkillCategory::Language,
        },
        Skill {
            name: "Ethereum".to_string(),
            level: 95,
            category: SkillCategory::Blockchain,
        },
        Skill {
            name: "Solana".to_string(),
            level: 85,
            category: SkillCategory::Blockchain,
        },
    ]
}

fn get_mock_experiences() -> Vec<Experience> {
    vec![Experience {
        company: "Web3 Startup".to_string(),
        role: "Senior Blockchain Engineer".to_string(),
        duration: "2022 - Present".to_string(),
        description: "Leading smart contract development and architecture".to_string(),
        achievements: vec![
            "Built DeFi protocol handling $50M+ TVL".to_string(),
            "Reduced gas costs by 40% through optimization".to_string(),
        ],
    }]
}
