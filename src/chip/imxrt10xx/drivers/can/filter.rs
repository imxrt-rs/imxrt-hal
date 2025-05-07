//! Filter bank API.

/// Decodes value of the ID Extended (IDE) and Remote Transmission Request (RTR) bits
///
/// TODO: Break this up into seperate RTR and IDE structs
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub enum FlexCanIde {
    /// IDE = 0, RTR = 0
    #[default]
    None = 0,
    /// IDE = 1, RTR = 0
    Ext = 1,
    /// IDE = 0, RTR = 1
    Rtr = 2,
    /// IDE = 1, RTR = 1
    Std = 3,
    /// Unused(?) variant
    Inactive,
}

/// Filter enable/disable.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub enum FlexCanFlten {
    /// Accept all CAN frames (filtering is disabled)
    AcceptAll = 0,
    /// Reject all CAN frames that do not match the filter exactly
    #[default]
    RejectAll = 1,
}

/// FlexCAN Filter.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub struct FlexCanFilter {
    /// Mailbox ID that this filter will apply to
    pub filter_id: u8,
    /// Frame ID
    pub id: u32,
    /// Extended ID Bit (IDE)
    pub ide: FlexCanIde,
    /// Remote Frame Bit (RTR)
    pub remote: FlexCanIde,
}

impl FlexCanFilter {
    /// Create a new [`FlexCanFilter`].
    pub fn new(filter_id: u8, id: u32, ide: FlexCanIde, remote: FlexCanIde) -> Self {
        Self {
            filter_id,
            id,
            ide,
            remote,
        }
    }
}
