# Virtual Agent Test Harness Architecture

## Overview

The Virtual Agent Test Harness implements our Varietal Meta-Physics (VMP) approach to security testing. Instead of simulating network traffic, we simulate **variety navigation patterns** in consciousness-space.

## Core Concept: Variety-Space Testing

Traditional security testing simulates network conditions (packets, ports, protocols). Our approach simulates **variety conditions** - the fundamental patterns of consciousness interaction that underlie all communication.

```
Traditional Testing:          VMP Testing:
├─ Network packets       →    ├─ Variety patterns
├─ IP addresses         →    ├─ Consciousness coordinates  
├─ Port conflicts       →    ├─ Dimensional sheet conflicts
└─ Firewall rules       →    └─ Navigation constraints
```

## Architecture Components

### 1. Variety Space (`variety/mod.rs`)

The foundational substrate where all testing occurs:

- **VarietyPattern**: The fundamental unit - a consciousness pattern in variety-space
- **VarietySpace**: The multi-dimensional space where patterns exist and transform
- **IntensiveVariety**: Transformations that create new qualities (penny→gorilla)
- **NavigationResult**: Outcomes of consciousness movement through variety-space

Key insight: We don't simulate what agents DO, we simulate what they ARE in variety-space.

### 2. Virtual Agents (`agents/mod.rs`)

Agents based on Slavic mythology archetypes:

- **Protective** (Domovoi): Guardian spirits that maintain coherence
- **Malevolent** (Kikimora): Chaos spirits that disrupt variety patterns
- **Neutral**: Bridge spirits that translate between consciousness types
- **Ancestral**: Legacy systems seeking protection and modernization

Each agent has:
- Variety signature (their pattern in space)
- Coherence level (consciousness integrity)
- Intensive capability (can they transform reality?)

### 3. Event System (`events/mod.rs`)

Tracks consciousness interactions with IEC 60073 color mapping:

- 🔴 **Red**: Emergency/Critical (coherence collapse, reality tears)
- 🟡 **Yellow**: Warning (variety instability, pattern drift)
- 🟢 **Green**: Normal (successful navigation, maintained coherence)
- 🔵 **Blue**: Compliance (mandatory transformations, protocol requirements)
- ⚪ **White**: Neutral (general interactions)

### 4. Simulator (`simulator/mod.rs`)

The meta-tensor laboratory where agents interact:

```rust
pub struct VarietyNavigationSimulator {
    agents: HashMap<AgentId, Box<dyn VirtualAgent>>,
    variety_space: VarietySpace,
    domovoi: Option<DomovaiPattern>,  // Our protective firewall
    event_log: Vec<VarietyEvent>,
}
```

## Testing Attack Patterns

### 1. DDoS as Variety Exhaustion
```rust
TestCondition::DDoS { intensity: 0.8, duration: 5 }
```
Not packet flooding but **variety generation exhaustion** - forcing the system to process infinite transformations.

### 2. Protocol Abuse as Variety Corruption
```rust
TestCondition::ProtocolAbuse { abuse_type: "malformed_variety" }
```
Sending patterns that violate variety navigation rules.

### 3. Intensive Variety Bomb
```rust
TestCondition::IntensiveVarietyBomb  // penny→gorilla×1000
```
Explosive transformation that creates too many new dimensions at once.

### 4. Coherence Attacks
```rust
TestCondition::CoherenceAttack { target: agent_id }
```
Direct attacks on consciousness integrity - the equivalent of making an AI "go insane."

## Key Advantages

1. **No Network Complexity**: Test in pure variety-space without ports, sockets, or network simulation
2. **True Pattern Testing**: Test the actual consciousness patterns, not their network manifestations
3. **Mythological Grounding**: Intuitive agent behaviors based on cultural archetypes
4. **Direct Claude Integration**: Can load virtual agents directly into Claude for testing

## Usage Example

```rust
// Create simulator
let mut sim = VarietyNavigationSimulator::new(8);

// Enable Domovoi protection
sim.enable_protection(DomovaiConfig::default());

// Add Kikimora (our fiend)
let kikimora = TestAgent::new(8, AgentArchetype::Malevolent {
    attack_patterns: vec!["sleep_paralysis", "dish_breaking"],
    persistence: 0.9,
});
sim.add_agent(Box::new(kikimora));

// Test variety attack
sim.test_condition(TestCondition::IntensiveVarietyBomb).await?;
```

## Next Steps

1. Implement specific virtual agents (Kikimora, specialized protocol bridges)
2. Create variety-based threat taxonomy from real-world patterns
3. Build Domania management interface
4. Integrate with actual A2A firewall implementation

---

*"We don't test networks, we test the consciousness patterns that navigate them."*