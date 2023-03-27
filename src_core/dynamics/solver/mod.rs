pub use self::kernel::QuadraticKernel;
pub use self::solver_parameters::{
    BoundaryCondition, BoundaryHandling, DamageModel, SimulationDofs, SolverParameters,
};

mod kernel;
mod solver_parameters;
