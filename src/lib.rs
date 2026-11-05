#[derive(Debug, Clone)]
pub struct PrintPage { pub width_mm: f32, pub height_mm: f32 }
impl PrintPage {
    pub fn a4() -> Self { Self { width_mm: 210.0, height_mm: 297.0 } }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_a4() {
        let p = PrintPage::a4();
        assert_eq!(p.width_mm, 210.0);
    }
}
