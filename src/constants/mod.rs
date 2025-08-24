pub mod magics_bishop;
pub mod magics_rook;

#[derive(Copy, Clone)]
pub struct Magic {
    pub magic: u64,
    pub mask: u64,
    pub shift: u8,
    pub offset: usize,
}
