//! **emergence-runtime** – Dynamic behavior composition and execution engine for EMERGENCE.

use emergence_physics::PhysicsEngine;
use emergence_nervous_system::NervousSystem;
use emergence_memory::MemorySubstrate;

pub struct ExecutionEngine {
    pub physics: PhysicsEngine,
    pub nervous_system: NervousSystem,
    pub memory: MemorySubstrate,
}

impl ExecutionEngine {
    pub async fn new() -> anyhow::Result<Self> {
        Ok(Self {
            physics: PhysicsEngine::new().await?,
            nervous_system: NervousSystem::new(),
            memory: MemorySubstrate::new(),
        })
    }
    
    /// Get physics engine for debugging
    pub fn get_physics_engine(&self) -> &PhysicsEngine {
        &self.physics
    }
    
    /// Get nervous system for debugging
    pub fn get_nervous_system(&self) -> &NervousSystem {
        &self.nervous_system
    }
    
    /// Get memory substrate for debugging
    pub fn get_memory_substrate(&self) -> &MemorySubstrate {
        &self.memory
    }
}