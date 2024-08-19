use super::*;

/// Returns the FlexCAN clock gate locator.
#[inline(always)]
pub const fn can<const N: u8>() -> Locator
where
    ral::can::Instance<N>: ral::Valid,
{
    [locator(CCGR0, CG7), locator(CCGR0, CG9)][if N == ral::SOLE_INSTANCE {
        N as usize
    } else {
        N as usize - 1
    }]
}

/// Returns the FlexCAN Peripheral Engine gate locator.
#[inline(always)]
pub const fn can_pe<const N: u8>() -> Locator
where
    ral::can::Instance<N>: ral::Valid,
{
    [locator(CCGR0, CG8), locator(CCGR0, CG10)][if N == ral::SOLE_INSTANCE {
        N as usize
    } else {
        N as usize - 1
    }]
}