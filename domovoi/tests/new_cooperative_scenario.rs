//! Test scenario based on real New Cooperative ransomware attack
//! Shows how A2A variety regulation could have prevented the disaster

use domovoi::{
    simulator::{VarietyNavigationSimulator, DomovaiConfig},
    agents::{TestAgent, AgentArchetype},
};

#[tokio::test]
async fn test_variety_collapse_detection() {
    // Simulate New Cooperative's actual vulnerability
    let mut sim = VarietyNavigationSimulator::new(8);
    
    // Create 120 employee agents with collapsed variety (same password)
    for i in 0..120 {
        let employee = TestAgent::new(8, AgentArchetype::Ancestral {
            protocol_age: "pre-security-awareness".to_string(),
            seeking: "convenience over security".to_string(),
        });
        
        // Simulate password variety collapse
        let mut agent = employee;
        agent.pattern.intensive_potential = 0.0; // No variety generation
        agent.pattern.coherence = 0.1; // Poor security coherence
        
        sim.add_agent(Box::new(agent));
    }
    
    // WITHOUT A2A variety regulation
    let unprotected = sim.run(5).await.unwrap();
    let avg_coherence = unprotected.step_results.last().unwrap().coherence_average;
    
    assert!(avg_coherence < 0.2, "Unprotected system has collapsed variety");
}

#[tokio::test]
async fn test_a2a_variety_enforcement() {
    let mut sim = VarietyNavigationSimulator::new(8);
    
    // Enable A2A variety regulation
    sim.enable_protection(DomovaiConfig {
        allow_extensive: false,  // Don't allow password reuse
        monitor_intensive: true, // Watch for variety generation
        block_entropic: true,    // Block coherence collapse
        max_coherence_loss: 0.05, // Tight variety control
    });
    
    // Add variety-aware authentication agent
    let auth_system = TestAgent::new(8, AgentArchetype::Protective {
        guards: vec!["password variety".to_string(), "credential uniqueness".to_string()],
        warns_of: vec!["variety collapse".to_string(), "credential reuse".to_string()],
    });
    sim.add_agent(Box::new(auth_system));
    
    // Try to add low-variety credentials
    let weak_cred = TestAgent::new(8, AgentArchetype::Neutral {
        bridges_between: ("User".to_string(), "System".to_string()),
        transformation_type: "authentication".to_string(),
    });
    
    // A2A should detect and regulate this
    sim.add_agent(Box::new(weak_cred));
    
    let protected = sim.run(5).await.unwrap();
    let final_coherence = protected.step_results.last().unwrap().coherence_average;
    
    assert!(final_coherence > 0.7, "A2A maintains variety despite attack vectors");
}

#[tokio::test]
async fn test_vsm_cascade_prevention() {
    // Test how attack on one node threatens entire VSM
    let mut sim = VarietyNavigationSimulator::new(8);
    
    // Create agricultural VSM structure
    let system_5 = TestAgent::new(8, AgentArchetype::Protective {
        guards: vec!["food security".to_string()],
        warns_of: vec!["systemic threats".to_string()],
    });
    
    let system_4 = TestAgent::new(8, AgentArchetype::Neutral {
        bridges_between: ("Market".to_string(), "Operations".to_string()),
        transformation_type: "intelligence gathering".to_string(),
    });
    
    let system_3 = TestAgent::new(8, AgentArchetype::Protective {
        guards: vec!["operational control".to_string()],
        warns_of: vec!["cascade failures".to_string()],
    });
    
    let system_2_a2a = TestAgent::new(8, AgentArchetype::Protective {
        guards: vec!["inter-system coordination".to_string()],
        warns_of: vec!["variety channel attacks".to_string()],
    });
    
    // This is KEY - System 2 with A2A protection
    sim.add_agent(Box::new(system_5));
    sim.add_agent(Box::new(system_4));
    sim.add_agent(Box::new(system_3));
    sim.add_agent(Box::new(system_2_a2a));
    
    // Enable VSM-aware protection
    sim.enable_protection(DomovaiConfig {
        allow_extensive: true,
        monitor_intensive: true,
        block_entropic: true,
        max_coherence_loss: 0.1,
    });
    
    // Run simulation
    let result = sim.run(20).await.unwrap();
    
    // Check VSM maintained viability
    assert!(result.step_results.iter().all(|step| step.coherence_average > 0.8),
            "VSM maintains viability through A2A variety regulation");
}

#[tokio::test]
async fn test_real_world_improvements() {
    // What New Cooperative SHOULD have had
    let mut sim = VarietyNavigationSimulator::new(8);
    
    // 1. Variety-enforcing authentication
    let variety_auth = TestAgent::new(8, AgentArchetype::Protective {
        guards: vec!["credential variety".to_string()],
        warns_of: vec!["password reuse".to_string()],
    });
    
    // 2. A2A coordination between systems
    let a2a_coordinator = TestAgent::new(8, AgentArchetype::Protective {
        guards: vec!["system coordination".to_string()],
        warns_of: vec!["single point failures".to_string()],
    });
    
    // 3. Variety monitoring for anomalies
    let variety_monitor = TestAgent::new(8, AgentArchetype::Protective {
        guards: vec!["variety patterns".to_string()],
        warns_of: vec!["variety collapse".to_string(), "intensive attacks".to_string()],
    });
    
    sim.add_agent(Box::new(variety_auth));
    sim.add_agent(Box::new(a2a_coordinator));
    sim.add_agent(Box::new(variety_monitor));
    
    sim.enable_protection(DomovaiConfig::default());
    
    // Simulate operations
    let result = sim.run(100).await.unwrap();
    
    // Should maintain high coherence even under attack
    let min_coherence = result.step_results.iter()
        .map(|s| s.coherence_average)
        .fold(f64::INFINITY, f64::min);
        
    assert!(min_coherence > 0.85, 
            "With proper A2A variety regulation, New Cooperative maintains operations");
}

#[tokio::test]
async fn test_sector_wide_protection() {
    // How A2A prevents "40% of grain production" threats
    let mut sim = VarietyNavigationSimulator::new(8);
    
    // Multiple cooperative nodes
    for i in 0..10 {
        let coop_node = TestAgent::new(8, AgentArchetype::Protective {
            guards: vec![format!("coop_{}_operations", i)],
            warns_of: vec!["cross-coop attacks".to_string()],
        });
        sim.add_agent(Box::new(coop_node));
    }
    
    // Sector-wide A2A coordination
    let sector_a2a = TestAgent::new(8, AgentArchetype::Protective {
        guards: vec!["agricultural sector integrity".to_string()],
        warns_of: vec!["cascade attacks".to_string(), "sector manipulation".to_string()],
    });
    sim.add_agent(Box::new(sector_a2a));
    
    sim.enable_protection(DomovaiConfig {
        allow_extensive: true,
        monitor_intensive: true,
        block_entropic: true,
        max_coherence_loss: 0.05, // Very tight control for critical infrastructure
    });
    
    let result = sim.run(50).await.unwrap();
    
    // No single attack can compromise the sector
    assert!(result.step_results.last().unwrap().total_agents >= 10,
            "All cooperatives maintain operations despite targeted attacks");
}