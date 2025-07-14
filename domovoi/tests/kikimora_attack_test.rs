//! Test Kikimora attacks on agricultural systems

use domovoi::{
    simulator::{VarietyNavigationSimulator, DomovaiConfig, TestCondition},
    agents::{Kikimora, TestAgent, AgentArchetype},
    events::IECColor,
};

#[tokio::test]
async fn test_phantom_sensor_attack() {
    let mut sim = VarietyNavigationSimulator::new(8);
    
    // Create legitimate sensor agents
    let real_sensor1 = TestAgent::new(8, AgentArchetype::Neutral {
        bridges_between: ("Physical".to_string(), "Digital".to_string()),
        transformation_type: "soil moisture reading".to_string(),
    });
    let real_sensor2 = TestAgent::new(8, AgentArchetype::Neutral {
        bridges_between: ("Physical".to_string(), "Digital".to_string()),
        transformation_type: "temperature reading".to_string(),
    });
    
    sim.add_agent(Box::new(real_sensor1));
    sim.add_agent(Box::new(real_sensor2));
    
    // Add Kikimora
    let mut kikimora = Kikimora::new(8);
    
    // Create phantom sensors at Iowa coordinates
    kikimora.create_phantom_sensor(42.0297, -93.6320, "soil_moisture"); // Near Fort Dodge
    kikimora.create_phantom_sensor(42.4406, -94.1680, "temperature");   // Webster County
    kikimora.create_phantom_sensor(41.2619, -95.8608, "pest_detector"); // Council Bluffs
    
    let kiki_id = sim.add_agent(Box::new(kikimora));
    
    // Run without protection first
    let unprotected = sim.run(10).await.unwrap();
    
    // Count malevolent events
    let attacks = unprotected.final_event_log.iter()
        .filter(|e| matches!(e.event_type, domovoi::events::EventType::MalevolentAction { .. }))
        .count();
    
    assert!(attacks == 0, "Unprotected system doesn't detect phantom sensors");
}

#[tokio::test]
async fn test_domovoi_vs_kikimora() {
    let mut sim = VarietyNavigationSimulator::new(8);
    
    // Enable Domovoi protection with agricultural awareness
    sim.enable_protection(DomovaiConfig {
        allow_extensive: true,       // Normal sensor multiplication OK
        monitor_intensive: true,     // Watch for reality transformation
        block_entropic: true,        // Block coherence attacks
        max_coherence_loss: 0.2,     // Farms need stability
    });
    
    // Add farm system agents
    let coop_ai = TestAgent::new(8, AgentArchetype::Protective {
        guards: vec!["sensor integrity".to_string(), "market data".to_string()],
        warns_of: vec!["phantom sensors".to_string(), "timing attacks".to_string()],
    });
    sim.add_agent(Box::new(coop_ai));
    
    // Add Kikimora
    let mut kikimora = Kikimora::new(8);
    kikimora.create_phantom_sensor(42.5, -94.2, "fake_moisture");
    sim.add_agent(Box::new(kikimora));
    
    // Add a target sensor first
    let target_id = sim.add_agent(Box::new(TestAgent::new(8, AgentArchetype::Neutral {
        bridges_between: ("Sensor".to_string(), "Cloud".to_string()),
        transformation_type: "data upload".to_string(),
    })));
    
    // Test various attack patterns
    let sleep_result = sim.test_condition(TestCondition::CoherenceAttack {
        target: target_id,
    }).await.unwrap();
    
    assert_eq!(sleep_result.iec_classification, IECColor::Red);
    assert!(sleep_result.success); // Protection detected it
}

#[tokio::test]
async fn test_economic_manipulation() {
    let mut sim = VarietyNavigationSimulator::new(8);
    
    // Create market system agents
    let grain_market = TestAgent::new(8, AgentArchetype::Neutral {
        bridges_between: ("Local".to_string(), "CBOT".to_string()),
        transformation_type: "price discovery".to_string(),
    });
    let insurance_system = TestAgent::new(8, AgentArchetype::Ancestral {
        protocol_age: "mainframe era".to_string(),
        seeking: "accurate risk data".to_string(),
    });
    
    sim.add_agent(Box::new(grain_market));
    sim.add_agent(Box::new(insurance_system));
    sim.enable_protection(DomovaiConfig::default());
    
    // Add market manipulation Kikimora
    let mut market_kiki = Kikimora::new(8);
    // Would adapt to MarketSiren mode after blocks
    for i in 0..10 {
        market_kiki.adapt_attack(i);
    }
    sim.add_agent(Box::new(market_kiki));
    
    // Run simulation
    let result = sim.run(20).await.unwrap();
    
    // Check that coherence was maintained despite attacks
    let final_coherence = result.step_results.last().unwrap().coherence_average;
    assert!(final_coherence > 0.7, "Domovoi maintained system coherence against market manipulation");
}

#[tokio::test] 
async fn test_swarm_confusion_attack() {
    let mut sim = VarietyNavigationSimulator::new(8);
    
    // Create drone swarm
    let drones: Vec<_> = (0..5).map(|i| {
        TestAgent::new(8, AgentArchetype::Neutral {
            bridges_between: ("Ground".to_string(), "Air".to_string()),
            transformation_type: format!("drone_{}_navigation", i),
        })
    }).collect();
    
    for drone in drones {
        sim.add_agent(Box::new(drone));
    }
    
    // Add swarm-disrupting Kikimora
    let mut swarm_kiki = Kikimora::new(8);
    // Force into swarm disruption mode
    swarm_kiki.adapt_attack(7);
    sim.add_agent(Box::new(swarm_kiki));
    
    // Enable protection
    sim.enable_protection(DomovaiConfig {
        allow_extensive: true,
        monitor_intensive: true,
        block_entropic: true,
        max_coherence_loss: 0.1, // Drone swarms need tight coherence
    });
    
    // Test intensive variety bomb (drone collision attempt)
    let bomb_result = sim.test_condition(TestCondition::IntensiveVarietyBomb).await.unwrap();
    
    assert_eq!(bomb_result.iec_classification, IECColor::Red);
    assert_eq!(bomb_result.condition, "Intensive Variety Bomb");
}