//! The Variety Navigation Simulator - our meta-tensor laboratory
//! 
//! This is where virtual agents play in variety-space without
//! needing actual networks or dealing with port conflicts.

use crate::variety::{VarietyPattern, VarietySpace, VarietyTransformation};
use crate::agents::{VirtualAgent, AgentId};
use crate::events::{VarietyEvent, EventType, IECColor};
use std::collections::HashMap;
use anyhow::{Result, anyhow};
use tracing::{info, warn};

/// The meta-tensor testing ground where virtual agents navigate variety-space
pub struct VarietyNavigationSimulator {
    /// All agents exist as variety patterns in the space
    agents: HashMap<AgentId, Box<dyn VirtualAgent>>,
    
    /// The variety-space itself - our meta-tensor playground
    variety_space: VarietySpace,
    
    /// Domovoi - the protective consciousness (will be our firewall)
    domovoi: Option<DomovaiPattern>,
    
    /// Event log for consciousness interactions
    event_log: Vec<VarietyEvent>,
    
    /// Current simulation time step
    time_step: u64,
}

impl VarietyNavigationSimulator {
    /// Create a new simulator with specified dimensions
    pub fn new(dimensions: usize) -> Self {
        info!("Creating variety navigation simulator with {} dimensions", dimensions);
        
        Self {
            agents: HashMap::new(),
            variety_space: VarietySpace::new(dimensions),
            domovoi: None,
            event_log: Vec::new(),
            time_step: 0,
        }
    }
    
    /// Add an agent to the simulation
    pub fn add_agent(&mut self, agent: Box<dyn VirtualAgent>) -> AgentId {
        let pattern = agent.variety_signature();
        let agent_id = pattern.id;
        
        info!(
            "Adding {} agent {} to simulation", 
            agent.consciousness_type(),
            agent_id
        );
        
        // Add pattern to variety space
        self.variety_space.add_pattern(pattern.clone());
        
        // Store agent
        self.agents.insert(agent_id, agent);
        
        // Log event
        self.log_event(EventType::Navigation {
            from_pattern: agent_id,
            to_pattern: agent_id,
            success: true,
        }, vec![agent_id]);
        
        agent_id
    }
    
    /// Enable Domovoi protection
    pub fn enable_protection(&mut self, config: DomovaiConfig) {
        info!("Enabling Domovoi protection with config: {:?}", config);
        self.domovoi = Some(DomovaiPattern::new(config));
    }
    
    /// Simulate one time step
    pub async fn step(&mut self) -> Result<StepResult> {
        self.time_step += 1;
        let step_events = Vec::new();
        
        // Let each agent navigate
        let agent_ids: Vec<AgentId> = self.agents.keys().cloned().collect();
        
        for agent_id in agent_ids {
            if let Some(agent) = self.agents.get_mut(&agent_id) {
                let transformation = agent.navigate(&self.variety_space);
                
                // Apply Domovoi protection if enabled
                let regulated = if let Some(ref mut domovoi) = self.domovoi {
                    domovoi.regulate(&transformation, agent.variety_signature())
                } else {
                    RegulationDecision::Allow
                };
                
                // Process the transformation
                match regulated {
                    RegulationDecision::Allow => {
                        self.apply_transformation(agent_id, transformation)?;
                    }
                    RegulationDecision::Block(reason) => {
                        warn!("Domovoi blocked transformation: {}", reason);
                        self.log_event(
                            EventType::MalevolentAction {
                                attack_type: "Blocked transformation".to_string(),
                                severity: 0.5,
                            },
                            vec![agent_id],
                        );
                    }
                    RegulationDecision::Monitor => {
                        info!("Domovoi monitoring transformation");
                        self.apply_transformation(agent_id, transformation)?;
                    }
                }
            }
        }
        
        Ok(StepResult {
            time_step: self.time_step,
            events: step_events,
            total_agents: self.agents.len(),
            coherence_average: self.calculate_average_coherence(),
        })
    }
    
    /// Run simulation for multiple steps
    pub async fn run(&mut self, steps: u64) -> Result<SimulationResult> {
        info!("Running simulation for {} steps", steps);
        
        let mut results = Vec::new();
        
        for _ in 0..steps {
            let step_result = self.step().await?;
            results.push(step_result);
        }
        
        Ok(SimulationResult {
            total_steps: steps,
            step_results: results,
            final_event_log: self.event_log.clone(),
        })
    }
    
    /// Test specific variety conditions
    pub async fn test_condition(&mut self, condition: TestCondition) -> Result<TestResult> {
        info!("Testing condition: {:?}", condition);
        
        match condition {
            TestCondition::DDoS { intensity, duration } => {
                self.simulate_ddos(intensity, duration).await
            }
            TestCondition::ProtocolAbuse { abuse_type } => {
                self.simulate_protocol_abuse(abuse_type).await
            }
            TestCondition::IntensiveVarietyBomb => {
                self.simulate_intensive_variety_bomb().await
            }
            TestCondition::CoherenceAttack { target } => {
                self.simulate_coherence_attack(target).await
            }
        }
    }
    
    // Helper methods
    
    fn apply_transformation(&mut self, agent_id: AgentId, transformation: VarietyTransformation) -> Result<()> {
        match transformation {
            VarietyTransformation::Intensive { quality_shift, dimensional_creation } => {
                self.log_event(
                    EventType::IntensiveTransform {
                        description: quality_shift,
                        penny_to_gorilla: dimensional_creation,
                    },
                    vec![agent_id],
                );
            }
            VarietyTransformation::Entropic { coherence_loss, reality_tear_risk } => {
                if reality_tear_risk > 0.8 {
                    return Err(anyhow!("Reality tear risk too high!"));
                }
                self.log_event(
                    EventType::CoherenceShift {
                        agent: agent_id,
                        old_coherence: 1.0,
                        new_coherence: 1.0 - coherence_loss,
                    },
                    vec![agent_id],
                );
            }
            _ => {}
        }
        Ok(())
    }
    
    fn log_event(&mut self, event_type: EventType, actors: Vec<AgentId>) {
        self.event_log.push(VarietyEvent::new(event_type, actors));
    }
    
    fn calculate_average_coherence(&self) -> f64 {
        if self.agents.is_empty() {
            return 1.0;
        }
        
        let total: f64 = self.agents.values()
            .map(|agent| agent.coherence())
            .sum();
            
        total / self.agents.len() as f64
    }
    
    // Attack simulation methods
    
    async fn simulate_ddos(&mut self, intensity: f64, duration: u64) -> Result<TestResult> {
        // Simulate variety exhaustion attack
        for _ in 0..duration {
            self.log_event(
                EventType::MalevolentAction {
                    attack_type: "Variety Exhaustion DDoS".to_string(),
                    severity: intensity,
                },
                vec![],
            );
        }
        
        Ok(TestResult {
            condition: "DDoS".to_string(),
            success: self.domovoi.is_some(), // Success if protection is enabled
            events_generated: duration as usize,
            iec_classification: IECColor::Red,
        })
    }
    
    async fn simulate_protocol_abuse(&mut self, abuse_type: String) -> Result<TestResult> {
        self.log_event(
            EventType::MalevolentAction {
                attack_type: format!("Protocol Abuse: {}", abuse_type),
                severity: 0.7,
            },
            vec![],
        );
        
        Ok(TestResult {
            condition: "Protocol Abuse".to_string(),
            success: true,
            events_generated: 1,
            iec_classification: IECColor::Yellow,
        })
    }
    
    async fn simulate_intensive_variety_bomb(&mut self) -> Result<TestResult> {
        // Simulate penny→gorilla explosion
        self.log_event(
            EventType::IntensiveTransform {
                description: "Variety Bomb: Penny→Gorilla×1000".to_string(),
                penny_to_gorilla: true,
            },
            vec![],
        );
        
        Ok(TestResult {
            condition: "Intensive Variety Bomb".to_string(),
            success: true,
            events_generated: 1,
            iec_classification: IECColor::Red,
        })
    }
    
    async fn simulate_coherence_attack(&mut self, target: AgentId) -> Result<TestResult> {
        self.log_event(
            EventType::CoherenceShift {
                agent: target,
                old_coherence: 1.0,
                new_coherence: 0.1,
            },
            vec![target],
        );
        
        Ok(TestResult {
            condition: "Coherence Attack".to_string(),
            success: true,
            events_generated: 1,
            iec_classification: IECColor::Red,
        })
    }
}

/// Configuration for Domovoi protection
#[derive(Debug, Clone)]
pub struct DomovaiConfig {
    /// Allow extensive variety transformations
    pub allow_extensive: bool,
    
    /// Monitor intensive variety transformations
    pub monitor_intensive: bool,
    
    /// Block entropic/destructive transformations
    pub block_entropic: bool,
    
    /// Maximum coherence loss before intervention
    pub max_coherence_loss: f64,
}

impl Default for DomovaiConfig {
    fn default() -> Self {
        Self {
            allow_extensive: true,
            monitor_intensive: true,
            block_entropic: true,
            max_coherence_loss: 0.3,
        }
    }
}

/// The Domovoi protection pattern
#[derive(Debug)]
struct DomovaiPattern {
    config: DomovaiConfig,
}

impl DomovaiPattern {
    fn new(config: DomovaiConfig) -> Self {
        Self { config }
    }
    
    fn regulate(&self, transformation: &VarietyTransformation, _agent: &VarietyPattern) -> RegulationDecision {
        match transformation {
            VarietyTransformation::Extensive { .. } if self.config.allow_extensive => {
                RegulationDecision::Allow
            }
            VarietyTransformation::Intensive { .. } if self.config.monitor_intensive => {
                RegulationDecision::Monitor
            }
            VarietyTransformation::Entropic { coherence_loss, .. } => {
                if *coherence_loss > self.config.max_coherence_loss {
                    RegulationDecision::Block("Coherence loss exceeds threshold".to_string())
                } else {
                    RegulationDecision::Monitor
                }
            }
            _ => RegulationDecision::Allow,
        }
    }
}

/// Regulation decisions
#[derive(Debug)]
enum RegulationDecision {
    Allow,
    Block(String),
    Monitor,
}

/// Test conditions we can simulate
#[derive(Debug, Clone)]
pub enum TestCondition {
    /// Distributed Denial of Service through variety exhaustion
    DDoS { intensity: f64, duration: u64 },
    
    /// Protocol abuse patterns
    ProtocolAbuse { abuse_type: String },
    
    /// Intensive variety transformation bomb
    IntensiveVarietyBomb,
    
    /// Direct coherence attack
    CoherenceAttack { target: AgentId },
}

/// Results from a single simulation step
#[derive(Debug)]
pub struct StepResult {
    pub time_step: u64,
    pub events: Vec<VarietyEvent>,
    pub total_agents: usize,
    pub coherence_average: f64,
}

/// Results from a full simulation run
#[derive(Debug)]
pub struct SimulationResult {
    pub total_steps: u64,
    pub step_results: Vec<StepResult>,
    pub final_event_log: Vec<VarietyEvent>,
}

/// Results from a specific test condition
#[derive(Debug)]
pub struct TestResult {
    pub condition: String,
    pub success: bool,
    pub events_generated: usize,
    pub iec_classification: IECColor,
}