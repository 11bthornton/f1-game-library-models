// Shared domain enums used by more than one packet type.
// Each file groups enums by domain. All derive TryFromPrimitive so TryFrom<u8> is free.

pub mod assists;
pub mod fia_flag;
pub mod lap;
pub mod nationality;
pub mod platform;
pub mod safety_car;
pub mod surface;
pub mod team;
pub mod track;
pub mod tyre;

pub use assists::TractionControl;
pub use fia_flag::FiaFlag;
pub use lap::{DriverStatus, PitStatus, ResultReason, ResultStatus, Sector};
pub use nationality::Nationality;
pub use platform::Platform;
pub use safety_car::SafetyCarType;
pub use surface::SurfaceType;
pub use team::Team;
pub use track::Track;
pub use tyre::{ActualTyreCompound, VisualTyreCompound};
