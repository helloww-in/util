#[derive(serde::Serialize, serde::Deserialize)]
pub struct Gun {
    damage: usize,
    size: (usize, usize),
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct knife {
    damage: usize,
    size: (usize, usize),
}
