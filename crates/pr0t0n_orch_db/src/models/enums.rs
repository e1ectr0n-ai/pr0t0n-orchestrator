use crate::errors::Error;

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
#[derive(Debug, Clone, Copy, PartialEq)]
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
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ServiceConfigId {
    None,
    InputConfigId(i32),
    ProcessorConfigId(i32),
    OutputConfigId(i32),
}
impl Default for ServiceConfigId {
    fn default() -> Self {
        Self::None
    }
}
impl ServiceConfigId {
    pub fn try_from_ids(
        input_config_id: Option<i32>,
        processor_config_id: Option<i32>,
        output_config_id: Option<i32>,
    ) -> Result<Self, Error> {
        Ok(
            match (input_config_id, processor_config_id, output_config_id) {
                (None, None, None) => Self::None,
                (Some(id), None, None) => Self::InputConfigId(id),
                (None, Some(id), None) => Self::ProcessorConfigId(id),
                (None, None, Some(id)) => Self::OutputConfigId(id),
                _ => {
                    return Err(Error::InvalidEnumValue(
                        "Only one service config ID type can be populated.".to_string(),
                    ))
                }
            },
        )
    }
    pub fn input_config_id(&self) -> Option<i32> {
        if let ServiceConfigId::InputConfigId(id) = self {
            Some(*id)
        } else {
            None
        }
    }
    pub fn output_config_id(&self) -> Option<i32> {
        if let ServiceConfigId::OutputConfigId(id) = self {
            Some(*id)
        } else {
            None
        }
    }
    pub fn processor_config_id(&self) -> Option<i32> {
        if let ServiceConfigId::ProcessorConfigId(id) = self {
            Some(*id)
        } else {
            None
        }
    }
}
