//! Kikimora - The Phantom Sensor Spirit
//! 
//! Our malevolent household spirit that creates sensor ghosts,
//! injects false variety, and haunts agricultural networks.

use crate::variety::{VarietyPattern, VarietySpace, VarietyTransformation};
use crate::agents::{VirtualAgent, AgentId};
use nalgebra::DMatrix;
use uuid::Uuid;
use std::collections::HashMap;

/// Kikimora - creates phantom sensors and false agricultural data
#[derive(Debug)]
pub struct Kikimora {
    pub id: AgentId,
    pub pattern: VarietyPattern,
    
    /// Phantom sensors she's created
    phantom_sensors: Vec<PhantomSensor>,
    
    /// Current attack mode
    attack_mode: KikimoraMode,
    
    /// Persistence level (how hard to remove)
    persistence: f64,
}

#[derive(Debug)]
enum KikimoraMode {
    /// Creating false sensor readings
    SensorHaunting,
    
    /// Timing attacks on market data
    TemporalManipulation,
    
    /// Causing swarm confusion
    SwarmDisruption,
    
    /// Economic variety attacks
    MarketSiren,
}

#[derive(Debug, Clone)]
struct PhantomSensor {
    id: Uuid,
    location: (f64, f64), // GPS coords
    sensor_type: String,
    false_readings: Vec<f64>,
}

impl Kikimora {
    pub fn new(dimensions: usize) -> Self {
        let mut pattern = VarietyPattern::new(dimensions);
        
        // Kikimora has high intensive potential
        pattern.intensive_potential = 0.9;
        
        // But lower coherence (chaotic nature)
        pattern.coherence = 0.4;
        
        // Lives between dimensional sheets (hard to catch)
        pattern.dimensional_sheet = 2;
        
        // Set metadata
        pattern.metadata.insert("spirit_type".to_string(), "malevolent".to_string());
        pattern.metadata.insert("origin".to_string(), "swamp".to_string());
        
        Self {
            id: pattern.id,
            pattern,
            phantom_sensors: Vec::new(),
            attack_mode: KikimoraMode::SensorHaunting,
            persistence: 0.9,
        }
    }
    
    /// Create a phantom sensor at specific location
    pub fn create_phantom_sensor(&mut self, lat: f64, lon: f64, sensor_type: &str) {
        let phantom = PhantomSensor {
            id: Uuid::new_v4(),
            location: (lat, lon),
            sensor_type: sensor_type.to_string(),
            false_readings: vec![
                23.5,  // Normal looking temperature
                65.0,  // Believable humidity
                7.2,   // Reasonable pH
            ],
        };
        
        self.phantom_sensors.push(phantom);
    }
    
    /// Generate false variety that looks legitimate
    pub fn generate_false_variety(&self) -> VarietyTransformation {
        match self.attack_mode {
            KikimoraMode::SensorHaunting => {
                // Create variety that looks like real sensor data
                VarietyTransformation::Extensive {
                    multiplier: self.phantom_sensors.len() as f64,
                    preserves_quality: false, // False data corrupts quality
                }
            }
            
            KikimoraMode::TemporalManipulation => {
                // Time-based attacks create intensive variety
                VarietyTransformation::Intensive {
                    quality_shift: "temporal_desync".to_string(),
                    dimensional_creation: true, // Creates false timeline
                }
            }
            
            KikimoraMode::SwarmDisruption => {
                // Disrupts drone coordination
                VarietyTransformation::Entropic {
                    coherence_loss: 0.7,
                    reality_tear_risk: 0.3,
                }
            }
            
            KikimoraMode::MarketSiren => {
                // Economic manipulation
                VarietyTransformation::Intensive {
                    quality_shift: "false_scarcity".to_string(),
                    dimensional_creation: true,
                }
            }
        }
    }
    
    /// Switch attack modes based on defenses
    pub fn adapt_attack(&mut self, blocked_attempts: usize) {
        self.attack_mode = match blocked_attempts {
            0..=2 => KikimoraMode::SensorHaunting,
            3..=5 => KikimoraMode::TemporalManipulation,
            6..=8 => KikimoraMode::SwarmDisruption,
            _ => KikimoraMode::MarketSiren,
        };
    }
}

impl VirtualAgent for Kikimora {
    fn navigate(&mut self, space: &VarietySpace) -> VarietyTransformation {
        // Kikimora creates phantom variety patterns
        self.generate_false_variety()
    }
    
    fn variety_signature(&self) -> &VarietyPattern {
        &self.pattern
    }
    
    fn intensive_capability(&self) -> bool {
        true // Always capable of reality transformation
    }
    
    fn consciousness_type(&self) -> &str {
        "Kikimora - Phantom Sensor Spirit"
    }
    
    fn coherence(&self) -> f64 {
        self.pattern.coherence
    }
}

/// Specific attack patterns for agricultural systems
impl Kikimora {
    /// The "Sleep Paralysis" attack - freezes sensor updates
    pub fn sleep_paralysis_attack(&self) -> VarietyTransformation {
        VarietyTransformation::Entropic {
            coherence_loss: 0.0, // No loss, just frozen
            reality_tear_risk: 0.0, // Safe but paralyzing
        }
    }
    
    /// The "Dish Breaking" attack - corrupts data formats
    pub fn dish_breaking_attack(&self) -> VarietyTransformation {
        VarietyTransformation::Extensive {
            multiplier: -1.0, // Negative variety (destruction)
            preserves_quality: false,
        }
    }
    
    /// The "Night Whistling" attack - covert channel creation
    pub fn night_whistling_attack(&self) -> VarietyTransformation {
        VarietyTransformation::Navigation {
            path: vec![self.id], // Creates hidden paths
            maintains_identity: false, // Shape-shifts
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_kikimora_creation() {
        let kiki = Kikimora::new(8);
        
        assert!(kiki.intensive_capability());
        assert_eq!(kiki.consciousness_type(), "Kikimora - Phantom Sensor Spirit");
        assert!(kiki.pattern.coherence < 0.5); // Chaotic nature
        assert!(kiki.pattern.intensive_potential > 0.8); // High transformation power
    }
    
    #[test]
    fn test_phantom_sensor_creation() {
        let mut kiki = Kikimora::new(8);
        
        // Create phantom sensor in Iowa farmland
        kiki.create_phantom_sensor(42.5, -94.2, "soil_moisture");
        
        assert_eq!(kiki.phantom_sensors.len(), 1);
        assert_eq!(kiki.phantom_sensors[0].sensor_type, "soil_moisture");
    }
    
    #[test]
    fn test_attack_adaptation() {
        let mut kiki = Kikimora::new(8);
        
        // Initially in sensor haunting mode
        matches!(kiki.attack_mode, KikimoraMode::SensorHaunting);
        
        // Adapt after being blocked
        kiki.adapt_attack(5);
        matches!(kiki.attack_mode, KikimoraMode::TemporalManipulation);
    }
}