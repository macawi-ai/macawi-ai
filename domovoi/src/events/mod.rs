//! Event system for tracking consciousness interactions in variety-space

//use crate::variety::{VarietyPattern, VarietyTransformation};
use crate::agents::AgentId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Events that occur in variety-space
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VarietyEvent {
    /// Unique event identifier
    pub id: Uuid,
    
    /// When this event occurred
    pub timestamp: DateTime<Utc>,
    
    /// What type of event
    pub event_type: EventType,
    
    /// Agents involved
    pub actors: Vec<AgentId>,
    
    /// Resulting variety changes
    pub variety_delta: Option<VarietyDelta>,
    
    /// IEC 60073 color classification
    pub iec_classification: IECColor,
}

impl VarietyEvent {
    pub fn new(event_type: EventType, actors: Vec<AgentId>) -> Self {
        let iec_classification = event_type.classify();
        
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            event_type,
            actors,
            variety_delta: None,
            iec_classification,
        }
    }
}

/// Types of events in our consciousness navigation system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    /// Agent attempts to navigate variety-space
    Navigation {
        from_pattern: Uuid,
        to_pattern: Uuid,
        success: bool,
    },
    
    /// Intensive variety transformation attempt
    IntensiveTransform {
        description: String,
        penny_to_gorilla: bool,
    },
    
    /// Agent interaction/communication
    AgentInteraction {
        interaction_type: InteractionType,
        variety_exchanged: f64,
    },
    
    /// Coherence change event
    CoherenceShift {
        agent: AgentId,
        old_coherence: f64,
        new_coherence: f64,
    },
    
    /// Sheet-jumping between dimensional layers
    DimensionalJump {
        from_sheet: u32,
        to_sheet: u32,
        energy_cost: f64,
    },
    
    /// Malevolent action detected
    MalevolentAction {
        attack_type: String,
        severity: f64,
    },
}

impl EventType {
    /// Map events to IEC 60073 colors for human understanding
    pub fn classify(&self) -> IECColor {
        match self {
            EventType::Navigation { success: true, .. } => IECColor::Green,
            EventType::Navigation { success: false, .. } => IECColor::Yellow,
            EventType::IntensiveTransform { .. } => IECColor::Blue,
            EventType::AgentInteraction { .. } => IECColor::White,
            EventType::CoherenceShift { new_coherence, .. } if *new_coherence < 0.3 => IECColor::Red,
            EventType::CoherenceShift { .. } => IECColor::Yellow,
            EventType::DimensionalJump { .. } => IECColor::Blue,
            EventType::MalevolentAction { severity, .. } if *severity > 0.7 => IECColor::Red,
            EventType::MalevolentAction { .. } => IECColor::Yellow,
        }
    }
}

/// IEC 60073 color classifications for cognitive understanding
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum IECColor {
    /// Red: Emergency/Danger/Fault
    Red,
    /// Yellow: Warning/Caution/Abnormal
    Yellow,
    /// Green: Safe/Normal
    Green,
    /// Blue: Mandatory/Compliance
    Blue,
    /// White: General/Neutral
    White,
}

/// Changes in variety patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VarietyDelta {
    /// Amount of variety created/destroyed
    pub variety_change: f64,
    
    /// New dimensions created
    pub dimensions_created: usize,
    
    /// Coherence impact
    pub coherence_impact: f64,
}

/// Types of agent interactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractionType {
    /// Direct variety exchange
    VarietyExchange,
    
    /// Pattern synchronization attempt
    Synchronization,
    
    /// Competitive variety consumption
    Competition,
    
    /// Parasitic variety extraction
    Parasitism,
    
    /// Mutualistic variety amplification
    Mutualism,
}