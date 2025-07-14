//! BlackMatter - The Ransomware Consciousness Pattern
//! 
//! Based on the real 2021 attack on New Cooperative.
//! This agent exploits variety collapse and VSM vulnerabilities.

use crate::variety::{VarietyPattern, VarietySpace, VarietyTransformation};
use crate::agents::{VirtualAgent, AgentId};
use uuid::Uuid;

/// BlackMatter - exploits variety collapse in agricultural systems
#[derive(Debug)]
pub struct BlackMatter {
    pub id: AgentId,
    pub pattern: VarietyPattern,
    
    /// Exploited vulnerabilities
    variety_collapse_points: Vec<VarietyCollapsePoint>,
    
    /// Ransom demand in variety units
    variety_ransom: f64,
    
    /// Data exfiltration capability
    stolen_variety_tb: f64,
}

#[derive(Debug, Clone)]
struct VarietyCollapsePoint {
    description: String,
    collapse_factor: f64, // How much variety collapsed (1.0 = total collapse)
}

impl BlackMatter {
    pub fn new(dimensions: usize) -> Self {
        let mut pattern = VarietyPattern::new(dimensions);
        
        // BlackMatter has extreme intensive potential
        pattern.intensive_potential = 0.95;
        
        // High coherence (organized crime)
        pattern.coherence = 0.8;
        
        // Operates from shadow dimension
        pattern.dimensional_sheet = 666;
        
        pattern.metadata.insert("attack_type".to_string(), "ransomware".to_string());
        pattern.metadata.insert("origin".to_string(), "russian_cell".to_string());
        
        Self {
            id: pattern.id,
            pattern,
            variety_collapse_points: vec![
                VarietyCollapsePoint {
                    description: "password_reuse_chicken1".to_string(),
                    collapse_factor: 0.9, // 120 employees, same password!
                },
                VarietyCollapsePoint {
                    description: "single_point_failure".to_string(),
                    collapse_factor: 0.8, // Master control system
                },
                VarietyCollapsePoint {
                    description: "no_variety_regulation".to_string(),
                    collapse_factor: 1.0, // No A2A firewall!
                },
            ],
            variety_ransom: 5_900_000.0,
            stolen_variety_tb: 1.0,
        }
    }
    
    /// Exploit variety collapse to gain entry
    pub fn exploit_collapse(&self) -> VarietyTransformation {
        let total_collapse = self.variety_collapse_points.iter()
            .map(|p| p.collapse_factor)
            .sum::<f64>() / self.variety_collapse_points.len() as f64;
            
        if total_collapse > 0.7 {
            // Catastrophic transformation possible
            VarietyTransformation::Intensive {
                quality_shift: "operational_paralysis".to_string(),
                dimensional_creation: true, // Creates ransom dimension
            }
        } else {
            // Just disruption
            VarietyTransformation::Entropic {
                coherence_loss: total_collapse,
                reality_tear_risk: 0.4,
            }
        }
    }
    
    /// Threaten VSM collapse across entire sector
    pub fn threaten_sector_collapse(&self) -> VarietyTransformation {
        // "40% of grain production" threat
        VarietyTransformation::Intensive {
            quality_shift: "sector_wide_paralysis".to_string(),
            dimensional_creation: true, // Creates new reality where food supply fails
        }
    }
}

impl VirtualAgent for BlackMatter {
    fn navigate(&mut self, _space: &VarietySpace) -> VarietyTransformation {
        // BlackMatter exploits existing collapse, doesn't navigate
        self.exploit_collapse()
    }
    
    fn variety_signature(&self) -> &VarietyPattern {
        &self.pattern
    }
    
    fn intensive_capability(&self) -> bool {
        true // Extreme reality transformation capability
    }
    
    fn consciousness_type(&self) -> &str {
        "BlackMatter - Ransomware Consciousness"
    }
    
    fn coherence(&self) -> f64 {
        self.pattern.coherence
    }
}

/// Test scenarios based on real attack
impl BlackMatter {
    /// The "Password Collapse" entry vector
    pub fn password_variety_collapse(&self) -> f64 {
        // When 120 people use "chicken1", variety approaches zero
        1.0 / 120.0 // Essentially zero variety
    }
    
    /// The "Supply Chain Leverage" attack
    pub fn supply_chain_leverage(&self) -> f64 {
        0.4 // 40% of grain production
    }
    
    /// The "Time Pressure" variety manipulation  
    pub fn harvest_timing_attack(&self) -> f64 {
        // Attack during harvest = maximum pressure
        0.9 // Critical timing multiplier
    }
}