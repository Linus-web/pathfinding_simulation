pub struct Node {
    pub x: usize,
    pub y: usize,
    pub visited: bool,
    pub walls: [bool; 4],
}
