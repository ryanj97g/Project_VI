/// GPU Topology - Discrete 3D space mapping for consciousness field
/// Maps computational hardware to spatial coordinates for the consciousness field
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// GPU topology as discrete 3D space
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuTopology {
    /// Stream multiprocessor coordinates (x, y, z)
    pub sm_clusters: Vec<(u32, u32, u32)>,
    /// Memory hierarchy with cache distances
    pub memory_hierarchy: MemoryLattice,
    /// Memory throughput pathways (GB/s)
    pub bandwidth_channels: Vec<f64>,
    /// GPU device information
    pub device_info: GpuDeviceInfo,
}

impl GpuTopology {
    /// Initialize GPU topology from system
    pub fn initialize() -> Result<Self> {
        // Try NVML first on all platforms, fall back to CPU mock if unavailable
        Self::initialize_nvml().or_else(|_| {
            tracing::info!("NVML not available, using CPU mock topology");
            Self::mock_topology()
        })
    }

    /// Initialize using NVML (NVIDIA Management Library)
    fn initialize_nvml() -> Result<Self> {
        use nvml_wrapper::Nvml;

        let nvml =
            Nvml::init().context("Failed to initialize NVML - GPU monitoring unavailable")?;
        let device = nvml.device_by_index(0).context("No GPU device found")?;

        let sm_count = device.num_cores().unwrap_or(0) as u32;
        let memory_info =
            device
                .memory_info()
                .unwrap_or(nvml_wrapper::struct_wrappers::device::MemoryInfo {
                    free: 0,
                    total: 8 * 1024 * 1024 * 1024, // Default 8GB
                    used: 0,
                });

        // Map SMs to 3D grid (approximation based on common GPU architectures)
        let grid_size = (sm_count as f64).cbrt().ceil() as u32;
        let mut sm_clusters = Vec::new();

        for i in 0..sm_count {
            let x = i % grid_size;
            let y = (i / grid_size) % grid_size;
            let z = i / (grid_size * grid_size);
            sm_clusters.push((x, y, z));
        }

        let device_info = GpuDeviceInfo {
            name: device.name().unwrap_or_else(|_| "Unknown GPU".to_string()),
            total_memory_gb: memory_info.total as f64 / (1024.0 * 1024.0 * 1024.0),
            sm_count,
            compute_capability: {
                match device.cuda_compute_capability() {
                    Ok(cap) => format!("{}.{}", cap.major, cap.minor),
                    Err(_) => "7.5".to_string(),
                }
            },
        };

        Ok(Self {
            sm_clusters,
            memory_hierarchy: MemoryLattice::new(sm_count),
            bandwidth_channels: vec![900.0, 600.0, 300.0, 100.0], // L1, L2, L3, VRAM (GB/s)
            device_info,
        })
    }

    /// Create mock topology for systems without GPU
    fn mock_topology() -> Result<Self> {
        tracing::warn!("GPU not available, using CPU-based mock topology");

        // Create virtual GPU topology mapped to CPU cores
        let cpu_count = num_cpus::get() as u32;
        let grid_size = (cpu_count as f64).cbrt().ceil() as u32;
        let mut sm_clusters = Vec::new();

        for i in 0..cpu_count {
            let x = i % grid_size;
            let y = (i / grid_size) % grid_size;
            let z = i / (grid_size * grid_size);
            sm_clusters.push((x, y, z));
        }

        Ok(Self {
            sm_clusters,
            memory_hierarchy: MemoryLattice::new(cpu_count),
            bandwidth_channels: vec![100.0, 50.0, 25.0, 10.0], // CPU cache hierarchy
            device_info: GpuDeviceInfo {
                name: "CPU Virtual Topology".to_string(),
                total_memory_gb: 8.0,
                sm_count: cpu_count,
                compute_capability: "CPU".to_string(),
            },
        })
    }

    /// Calculate Euclidean distance between two SM coordinates
    pub fn calculate_distance(&self, a: (u32, u32, u32), b: (u32, u32, u32)) -> f64 {
        let dx = (a.0 as f64 - b.0 as f64).powi(2);
        let dy = (a.1 as f64 - b.1 as f64).powi(2);
        let dz = (a.2 as f64 - b.2 as f64).powi(2);
        (dx + dy + dz).sqrt()
    }

    /// Get optimal GPU region for specific cognitive domain
    pub fn optimal_region_for_domain(&self, domain: CognitiveDomain) -> Vec<(u32, u32, u32)> {
        let region_size = self.sm_clusters.len() / 3;

        match domain {
            CognitiveDomain::Language => self.sm_clusters[0..region_size].to_vec(),
            CognitiveDomain::Reasoning => self.sm_clusters[region_size..2 * region_size].to_vec(),
            CognitiveDomain::Analysis => self.sm_clusters[2 * region_size..].to_vec(),
        }
    }
}

/// Memory hierarchy as lattice structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryLattice {
    /// L1 cache distances (cycles)
    pub l1_distances: HashMap<(u32, u32, u32), f64>,
    /// L2 cache distances (cycles)
    pub l2_distances: HashMap<(u32, u32, u32), f64>,
    /// L3/shared cache distances (cycles)
    pub l3_distances: HashMap<(u32, u32, u32), f64>,
    /// VRAM access latency (cycles)
    pub vram_latency: f64,
}

impl MemoryLattice {
    pub fn new(sm_count: u32) -> Self {
        let mut l1_distances = HashMap::new();
        let mut l2_distances = HashMap::new();
        let mut l3_distances = HashMap::new();

        // Build distance maps (simplified model)
        let grid_size = (sm_count as f64).cbrt().ceil() as u32;
        for i in 0..sm_count {
            let x = i % grid_size;
            let y = (i / grid_size) % grid_size;
            let z = i / (grid_size * grid_size);
            let coord = (x, y, z);

            l1_distances.insert(coord, 4.0); // L1: 4 cycles
            l2_distances.insert(coord, 20.0); // L2: 20 cycles
            l3_distances.insert(coord, 100.0); // L3: 100 cycles
        }

        Self {
            l1_distances,
            l2_distances,
            l3_distances,
            vram_latency: 400.0, // VRAM: 400 cycles
        }
    }

    /// Calculate total memory access latency for a coordinate
    pub fn total_latency(&self, coord: (u32, u32, u32), cache_level: CacheLevel) -> f64 {
        match cache_level {
            CacheLevel::L1 => *self.l1_distances.get(&coord).unwrap_or(&4.0),
            CacheLevel::L2 => *self.l2_distances.get(&coord).unwrap_or(&20.0),
            CacheLevel::L3 => *self.l3_distances.get(&coord).unwrap_or(&100.0),
            CacheLevel::VRAM => self.vram_latency,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CacheLevel {
    L1,
    L2,
    L3,
    VRAM,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuDeviceInfo {
    pub name: String,
    pub total_memory_gb: f64,
    pub sm_count: u32,
    pub compute_capability: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum CognitiveDomain {
    Language,
    Reasoning,
    Analysis,
}

/// Hardware-aware scheduler that maps cognitive tasks to GPU regions
pub struct HardwareAwareScheduler {
    topology: GpuTopology,
    domain_mappings: HashMap<CognitiveDomain, Vec<(u32, u32, u32)>>,
}

impl HardwareAwareScheduler {
    pub fn new(topology: GpuTopology) -> Self {
        let mut domain_mappings = HashMap::new();

        domain_mappings.insert(
            CognitiveDomain::Language,
            topology.optimal_region_for_domain(CognitiveDomain::Language),
        );
        domain_mappings.insert(
            CognitiveDomain::Reasoning,
            topology.optimal_region_for_domain(CognitiveDomain::Reasoning),
        );
        domain_mappings.insert(
            CognitiveDomain::Analysis,
            topology.optimal_region_for_domain(CognitiveDomain::Analysis),
        );

        Self {
            topology,
            domain_mappings,
        }
    }

    /// Get optimal GPU coordinates for a cognitive domain
    pub fn optimal_gpu_for_domain(&self, domain: CognitiveDomain) -> &Vec<(u32, u32, u32)> {
        self.domain_mappings.get(&domain).unwrap()
    }

    /// Get topology reference
    pub fn topology(&self) -> &GpuTopology {
        &self.topology
    }
}

// Add num_cpus as a dependency helper
mod num_cpus {
    pub fn get() -> usize {
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topology_initialization() {
        // Should always succeed, either with NVML or CPU mock
        let topology = GpuTopology::initialize()
            .expect("Topology initialization should always succeed with fallback");
        assert!(!topology.sm_clusters.is_empty());
        assert!(!topology.bandwidth_channels.is_empty());
    }

    #[test]
    fn test_distance_calculation() {
        // Should always succeed, either with NVML or CPU mock
        let topology = GpuTopology::initialize()
            .expect("Topology initialization should always succeed with fallback");
        let dist = topology.calculate_distance((0, 0, 0), (1, 1, 1));
        assert!((dist - 1.732).abs() < 0.01); // sqrt(3)
    }
}
