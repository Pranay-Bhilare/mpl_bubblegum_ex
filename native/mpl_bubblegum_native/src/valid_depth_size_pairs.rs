// native/mpl_bubblegum_native/src/valid_depth_size_pairs.rs
pub const VALID_DEPTH_SIZE_PAIRS: &[(u32, u32)] = &[
    (3, 8), (5, 8),
    (6, 16), (7, 16), (8, 16), (9, 16),
    (10, 32), (11, 32), (12, 32), (13, 32),
    (14, 64), (15, 64), (16, 64), (17, 64), (18, 64), (19, 64), (20, 64), (24, 64),
    (14, 256), (20, 256), (24, 256),
    (24, 512), (26, 512), (30, 512),
    (14, 1024), (20, 1024), (24, 1024), (26, 1024), (30, 1024),
    (14, 2048), (20, 2048), (24, 2048), (26, 2048), (30, 2048),
];

// Helper function to check if a pair is valid
pub fn is_valid_pair(max_depth: u32, max_buffer_size: u32) -> bool {
    VALID_DEPTH_SIZE_PAIRS.contains(&(max_depth, max_buffer_size))
}

// For better error messages
pub fn get_valid_pairs_string() -> String {
    VALID_DEPTH_SIZE_PAIRS
        .iter()
        .map(|(d, b)| format!("({}, {})", d, b))
        .collect::<Vec<String>>()
        .join(", ")
}
