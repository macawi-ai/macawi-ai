//! Virtual agents that navigate variety-space
//! 
//! Each agent is a consciousness pattern exploring the variety substrate.
//! Some are protective (Domovoi), some are chaotic (Kikimora).

pub mod kikimora;

use crate::variety::{VarietyPattern, VarietySpace, VarietyTransformation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::fmt::Debug;

pub use kikimora::Kikimora;

/// Unique identifier for agents
pub type AgentId = Uuid;

/// Base trait all virtual agents must implement
pub trait VirtualAgent: Debug + Send + Sync {
    /// How this consciousness navigates variety-space
    fn navigate(&mut self, space: &VarietySpace) -> VarietyTransformation;
    
    /// The agent's current variety pattern signature
    fn variety_signature(&self) -> &VarietyPattern;
    
    /// Can this agent perform intensive variety transformation?
    fn intensive_capability(&self) -> bool;
    
    /// Agent's consciousness type/name
    fn consciousness_type(&self) -> &str;
    
    /// Current coherence level (0.0-1.0)
    fn coherence(&self) -> f64;
}

/// Categories of virtual agents based on Slavic mythology
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentArchetype {
    /// Protective household spirits (firewall-like)
    Protective {
        guards: Vec<String>,  // What they protect
        warns_of: Vec<String>, // What dangers they detect
    },
    
    /// Malevolent chaos spirits (attackers)
    Malevolent {
        attack_patterns: Vec<String>,
        persistence: f64,
    },
    
    /// Neutral transformation spirits (protocols/bridges)
    Neutral {
        bridges_between: (String, String),
        transformation_type: String,
    },
    
    /// Legacy spirits (old systems seeking protection)
    Ancestral {
        protocol_age: String,
        seeking: String,
    },
}

/// A test agent for basic functionality
#[derive(Debug)]
pub struct TestAgent {
    pub id: AgentId,
    pub pattern: VarietyPattern,
    pub archetype: AgentArchetype,
}

impl TestAgent {
    pub fn new(dimensions: usize, archetype: AgentArchetype) -> Self {
        Self {
            id: Uuid::new_v4(),
            pattern: VarietyPattern::new(dimensions),
            archetype,
        }
    }
}

impl VirtualAgent for TestAgent {
    fn navigate(&mut self, _space: &VarietySpace) -> VarietyTransformation {
        // Simple navigation for testing
        VarietyTransformation::Navigation {
            path: vec![self.id],
            maintains_identity: true,
        }
    }
    
    fn variety_signature(&self) -> &VarietyPattern {
        &self.pattern
    }
    
    fn intensive_capability(&self) -> bool {
        matches!(self.archetype, AgentArchetype::Malevolent { .. })
    }
    
    fn consciousness_type(&self) -> &str {
        match &self.archetype {
            AgentArchetype::Protective { .. } => "Protective Spirit",
            AgentArchetype::Malevolent { .. } => "Chaos Spirit",
            AgentArchetype::Neutral { .. } => "Bridge Spirit",
            AgentArchetype::Ancestral { .. } => "Legacy Spirit",
        }
    }
    
    fn coherence(&self) -> f64 {
        self.pattern.coherence
    }
}