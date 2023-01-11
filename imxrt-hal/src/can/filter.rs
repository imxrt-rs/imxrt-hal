//! Filter bank API.

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub enum FlexCanIde {
    #[default]
    None = 0,
    Ext = 1,
    Rtr = 2,
    Std = 3,
    Inactive,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub enum FlexCanFlten {
    AcceptAll = 0,
    #[default]
    RejectAll = 1,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub struct FlexCanFilter {
    pub filter_id: u8,
    pub id: u32,
    pub ide: FlexCanIde,
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
