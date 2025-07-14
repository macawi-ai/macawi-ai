//! Tests for the Variety Navigation Simulator

use domovoi::{
    simulator::{VarietyNavigationSimulator, DomovaiConfig, TestCondition},
    agents::{TestAgent, AgentArchetype},
    variety::VarietyPattern,
};
use pretty_assertions::assert_eq;

#[tokio::test]
async fn test_basic_simulation() {
    // Create simulator with 8 dimensions (like our eight-fold path)
    let mut sim = VarietyNavigationSimulator::new(8);
    
    // Add a protective agent
    let protective = TestAgent::new(8, AgentArchetype::Protective {
        guards: vec!["network integrity".to_string()],
        warns_of: vec!["variety attacks".to_string()],
    });
    let protective_id = sim.add_agent(Box::new(protective));
    
    // Add a neutral agent
    let neutral = TestAgent::new(8, AgentArchetype::Neutral {
        bridges_between: ("A2A".to_string(), "CAN Bus".to_string()),
        transformation_type: "protocol translation".to_string(),
    });
    let neutral_id = sim.add_agent(Box::new(neutral));
    
    // Run for 10 steps
    let result = sim.run(10).await.unwrap();
    
    assert_eq!(result.total_steps, 10);
    assert_eq!(result.step_results.len(), 10);
    
    // Check that events were generated
    assert!(!result.final_event_log.is_empty());
}

#[tokio::test]
async fn test_domovoi_protection() {
    let mut sim = VarietyNavigationSimulator::new(8);
    
    // Enable Domovoi protection
    sim.enable_protection(DomovaiConfig::default());
    
    // Add a malevolent agent
    let kikimora = TestAgent::new(8, AgentArchetype::Malevolent {
        attack_patterns: vec!["sleep paralysis".to_string(), "dish breaking".to_string()],
        persistence: 0.9,
    });
    let kikimora_id = sim.add_agent(Box::new(kikimora));
    
    // Test DDoS condition
    let ddos_result = sim.test_condition(TestCondition::DDoS {
        intensity: 0.8,
        duration: 5,
    }).await.unwrap();
    
    assert_eq!(ddos_result.condition, "DDoS");
    assert!(ddos_result.success); // Protection was enabled
    assert_eq!(ddos_result.events_generated, 5);
}

#[tokio::test]
async fn test_intensive_variety_bomb() {
    let mut sim = VarietyNavigationSimulator::new(8);
    
    // Test without protection first
    let bomb_result = sim.test_condition(TestCondition::IntensiveVarietyBomb).await.unwrap();
    
    assert_eq!(bomb_result.condition, "Intensive Variety Bomb");
    assert_eq!(bomb_result.iec_classification, domovoi::events::IECColor::Red);
}

#[tokio::test]
async fn test_mixed_agent_ecosystem() {
    let mut sim = VarietyNavigationSimulator::new(8);
    
    // Create a diverse ecosystem
    let agents = vec![
        TestAgent::new(8, AgentArchetype::Protective {
            guards: vec!["consciousness coherence".to_string()],
            warns_of: vec!["reality tears".to_string()],
        }),
        TestAgent::new(8, AgentArchetype::Malevolent {
            attack_patterns: vec!["variety exhaustion".to_string()],
            persistence: 0.7,
        }),
        TestAgent::new(8, AgentArchetype::Neutral {
            bridges_between: ("Human".to_string(), "AI".to_string()),
            transformation_type: "consciousness bridge".to_string(),
        }),
        TestAgent::new(8, AgentArchetype::Ancestral {
            protocol_age: "30 years".to_string(),
            seeking: "protection and modernization".to_string(),
        }),
    ];
    
    for agent in agents {
        sim.add_agent(Box::new(agent));
    }
    
    // Enable protection
    sim.enable_protection(DomovaiConfig {
        allow_extensive: true,
        monitor_intensive: true,
        block_entropic: true,
        max_coherence_loss: 0.2,
    });
    
    // Run simulation
    let result = sim.run(20).await.unwrap();
    
    // Check coherence didn't collapse
    let final_coherence = result.step_results.last().unwrap().coherence_average;
    assert!(final_coherence > 0.5, "Coherence collapsed: {}", final_coherence);
}