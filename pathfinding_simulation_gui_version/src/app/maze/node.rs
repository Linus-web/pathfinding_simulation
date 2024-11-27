pub struct Node {
    pub x: usize,
    pub y: usize,
    pub visited: bool,
    pub walls: [bool; 4],
    pub is_current: bool,
    pub in_stack: bool,
}
