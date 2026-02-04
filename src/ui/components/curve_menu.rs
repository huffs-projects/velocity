use ratatui::layout::Rect;

pub const NUM_SLOTS: usize = 19;
pub const CURSOR_SLOT: usize = 9; // Center slot (0-indexed, so 9 is middle of 19)

pub fn calculate_curve_positions(area: Rect) -> Vec<(u16, u16)> {
    let mut positions = Vec::new();
    let globe_width_px = (area.width as f64 * 0.5) as u16;
    let center_y = area.height as f64 / 2.0;
    let y_range = area.height as f64 * 0.4; // Use 40% of height for curve
    
    // Calculate globe center (center of the left 50% area)
    let globe_center_x = globe_width_px as f64 / 2.0;
    let globe_center_y = center_y;
    
    // Estimate globe radius (accounting for 1.2x scale multiplier used during rendering)
    let base_radius = (globe_width_px.min(area.height) as f64 / 2.0) * 1.2;
    let offset_distance = 6.0; // Desired gap from globe edge
    
    // Calculate positions ensuring slot CURSOR_SLOT is at center_y
    // Distribute slots evenly along a curve from top to bottom
    // Use a set to track used Y positions to prevent overlaps
    let mut used_y_positions = std::collections::HashSet::new();
    
    for i in 0..NUM_SLOTS {
        // Calculate offset from center slot (CURSOR_SLOT)
        let offset_from_center = i as f64 - CURSOR_SLOT as f64;
        
        // Normalize to -1.0 to 1.0 range (relative to max offset)
        let max_offset = (NUM_SLOTS - 1) as f64 / 2.0;
        let normalized_offset = offset_from_center / max_offset;
        
        // Use linear distribution for even spacing: maps -1.0 to 1.0 linearly
        // This ensures consistent vertical spacing between all items, including at the extremes
        let curve_factor = normalized_offset;
        
        // Calculate Y position: center_y + curve_factor * y_range
        // curve_factor = -1 at top, 0 at center, +1 at bottom
        let mut y = (center_y + curve_factor * y_range) as u16;
        y = y.min(area.height.saturating_sub(1));
        
        // Ensure minimum spacing: if this Y position is already used, adjust it
        let mut adjusted_y = y;
        let mut offset = 0;
        while used_y_positions.contains(&adjusted_y) && offset < area.height {
            offset += 1;
            // Try positions above and below
            if i < CURSOR_SLOT {
                // For slots above cursor, prefer moving up
                adjusted_y = y.saturating_sub(offset);
            } else {
                // For slots below cursor, prefer moving down
                adjusted_y = (y + offset).min(area.height.saturating_sub(1));
            }
        }
        
        used_y_positions.insert(adjusted_y);
        
        // Calculate X position to maintain exactly 6 characters distance from globe surface
        // For a circular globe, at each Y position, we need to find the rightmost X of the globe
        // then place the menu item 6 characters to the right of that point
        let y_diff = adjusted_y as f64 - globe_center_y;
        let y_diff_squared = y_diff * y_diff;
        
        // Calculate the rightmost X position of the globe at this Y coordinate
        // For a circle: (x - center_x)^2 + (y - center_y)^2 = radius^2
        // Solving for the rightmost x: x = center_x + sqrt(radius^2 - (y - center_y)^2)
        let radius_squared = base_radius * base_radius;
        let x_offset_from_center_squared = radius_squared - y_diff_squared;
        
        let globe_rightmost_x = if x_offset_from_center_squared >= 0.0 {
            // Valid point on the circle - calculate rightmost X
            globe_center_x + x_offset_from_center_squared.sqrt()
        } else {
            // Y position is outside the globe vertically, use the right edge of globe area
            globe_width_px as f64
        };
        
        // Position menu item with varying offsets based on position from ends
        let effective_offset = if i == 0 || i == NUM_SLOTS - 1 {
            // First and last items: 4 characters (2 closer)
            offset_distance - 2.0
        } else if i == 1 || i == NUM_SLOTS - 2 {
            // Second and second-to-last items: 4 characters (2 closer)
            offset_distance - 2.0
        } else if i == 2 || i == NUM_SLOTS - 3 {
            // Third and third-to-last items: 5 characters (1 closer)
            offset_distance - 1.0
        } else if i == 3 || i == NUM_SLOTS - 4 {
            // Fourth and fourth-to-last items: 5 characters (1 closer)
            offset_distance - 1.0
        } else {
            // All other items: 6 characters (default)
            offset_distance
        };
        let x = (globe_rightmost_x + effective_offset) as u16;
        
        // Ensure X stays within screen bounds
        let x = x.min(area.width.saturating_sub(1));
        
        positions.push((x, adjusted_y));
    }
    positions
}
