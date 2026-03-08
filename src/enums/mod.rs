// Shared domain enums used by more than one packet type.
// Each file groups enums by domain. All derive TryFromPrimitive so TryFrom<u8> is free.

pub mod assists;
pub mod lap;
pub mod platform;
pub mod safety_car;
pub mod surface;
pub mod team;
pub mod tyre;

pub use assists::TractionControl;
pub use lap::{DriverStatus, PitStatus, ResultReason, ResultStatus, Sector};
pub use platform::Platform;
pub use safety_car::SafetyCarType;
pub use surface::SurfaceType;
pub use team::Team;
pub use tyre::{ActualTyreCompound, VisualTyreCompound};
