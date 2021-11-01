use crate::err::Error;

/// Shared interface for enums with corresponding strings.
pub trait StringEnum: Sized {
    fn as_str(&self) -> &'static str;
    fn from_str(text: &str) -> Result<Self, Error>;
}

/// Type of service.
#[derive(Debug, Clone, Copy)]
pub enum ServiceType {
    None,
    Input,
    Output,
    Processor,
}
impl StringEnum for ServiceType {
    fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Input => "input",
            Self::Output => "output",
            Self::Processor => "processor",
        }
    }
    fn from_str(text: &str) -> Result<Self, Error> {
        Ok(match text {
            "input" => Self::Input,
            "output" => Self::Output,
            "processor" => Self::Processor,
            _ => return Err(Error::InvalidEnumValue(text.to_string())),
        })
    }
}
impl Default for ServiceType {
    fn default() -> Self {
        Self::None
    }
}

/// Health status for a service.
#[derive(Debug, Clone, Copy)]
pub enum HealthStatus {
    Healthy,
    Disconnected,
    Warning,
    Critical,
}
impl StringEnum for HealthStatus {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::Disconnected => "disconnected",
            Self::Warning => "warning",
            Self::Critical => "critical",
        }
    }
    fn from_str(text: &str) -> Result<Self, Error> {
        Ok(match text {
            "healthy" => Self::Healthy,
            "disconnected" => Self::Disconnected,
            "warning" => Self::Warning,
            "critical" => Self::Critical,
            _ => return Err(Error::InvalidEnumValue(text.to_string())),
        })
    }
}
impl Default for HealthStatus {
    fn default() -> Self {
        Self::Healthy
    }
}

/// Health status for a service.
#[derive(Debug, Clone, Copy)]
pub enum ServiceConfigId {
    InputConfigId(i32),
    ServiceConfigId(i32),
    ProcessorConfigId(i32),
}
