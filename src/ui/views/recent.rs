use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Style, Modifier};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;
use crate::ui::components::GlobeComponent;
use crate::ui::components::{calculate_curve_positions, CURSOR_SLOT, NightSky};
use crate::ui::Theme;
use crate::recent_files::RecentFiles;

pub fn render_recent(frame: &mut Frame, globe: &mut GlobeComponent, recent_files: &RecentFiles, selected_index: Option<usize>, mut stars: Option<&mut NightSky>, theme: &Theme) {
    let area = frame.size();
    
    // Split: 50% globe (left), 50% content (right) - matching home view exactly
    let chunks = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);
    
    // Render globe on left (chunks[0]) - identical to home view
    let globe_area = chunks[0];
    let globe_width = globe_area.width as usize;
    let globe_height = globe_area.height as usize;
    
    // Temporarily increase scale to make globe slightly bigger (1.2x multiplier) - same as home view
    let original_scale = globe.get_scale();
    globe.set_scale(original_scale * 1.2);
    
    // Pre-render globe to get character buffer and identify occupied positions
    let mut occupied_positions = std::collections::HashSet::new();
    
    if let Ok(globe_frame) = globe.render(globe_width, globe_height) {
        // Track positions where globe has non-space characters
        for (y, row) in globe_frame.iter().enumerate() {
            if y >= globe_height {
                break;
            }
            for (x, &ch) in row.iter().take(globe_width).enumerate() {
                if ch != ' ' {
                    let abs_x = globe_area.x + x as u16;
                    let abs_y = globe_area.y + y as u16;
                    occupied_positions.insert((abs_x, abs_y));
                }
            }
        }
    }
    
    // Restore original scale
    globe.set_scale(original_scale);
    
    // Pre-calculate fixed positions along the right curve of the globe
    let positions = calculate_curve_positions(area);
    
    // Track file text positions
    let files = recent_files.get_files().unwrap_or_default();
    let total_files = files.len();
    
    // Use selected_index or default to 0
    let selected_idx = selected_index.unwrap_or(0).min(total_files.saturating_sub(1));
    
    if total_files > 0 {
        for (slot_index, &(x, y)) in positions.iter().enumerate() {
            // Calculate which file index should appear at this slot
            let file_index = if slot_index == CURSOR_SLOT {
                selected_idx
            } else if slot_index < CURSOR_SLOT {
                let offset = CURSOR_SLOT - slot_index;
                selected_idx.saturating_sub(offset)
            } else {
                let offset = slot_index - CURSOR_SLOT;
                selected_idx + offset
            };
            
            // Bounds checking
            if file_index >= total_files || y >= area.height || x >= area.width {
                continue;
            }
            
            let path = &files[file_index];
            let display_name: String = path
                .file_name()
                .and_then(|n| n.to_str())
                .map(|s| s.to_string())
                .unwrap_or_else(|| path.to_string_lossy().to_string());
            
            let display_text = if slot_index == CURSOR_SLOT {
                format!("{} <", display_name)
            } else {
                display_name.clone()
            };
            
            // Add all positions where file text will be rendered
            let text_length = display_text.chars().count() as u16;
            for offset_x in 0..text_length.min(area.width.saturating_sub(x)) {
                occupied_positions.insert((x + offset_x, y));
            }
        }
    }
    
    // Render stars FIRST as background layer, skipping occupied positions
    if let Some(ref mut stars) = stars {
        // Force reinitialize stars to ensure they use full screen dimensions
        // This handles the case where stars were initialized with old dimensions
        if area.width != stars.initialized_width || area.height != stars.initialized_height {
            stars.resize(area.width, area.height);
        }
        stars.render_with_occupied_positions(frame, area, &occupied_positions, theme);
    }
    
    // Now render globe normally (only write non-space characters to preserve stars in empty spaces)
    globe.set_scale(original_scale * 1.2);
    
    if let Ok(globe_frame) = globe.render(globe_width, globe_height) {
        for (y, row) in globe_frame.iter().enumerate() {
            if y >= globe_height {
                break;
            }
            for (x, &ch) in row.iter().take(globe_width).enumerate() {
                // Only write non-space characters to preserve stars in empty spaces
                if ch != ' ' {
                    let abs_x = globe_area.x + x as u16;
                    let abs_y = globe_area.y + y as u16;
                    frame.buffer_mut().get_mut(abs_x, abs_y).set_char(ch);
                }
            }
        }
    }
    
    // Restore original scale
    globe.set_scale(original_scale);
    
    // Render recent files list using fixed positions
    let files = recent_files.get_files().unwrap_or_default();
    let total_files = files.len();
    
    if total_files == 0 {
        return;
    }
    
    // Calculate which file appears at each slot based on fixed cursor position
    // The cursor stays at CURSOR_SLOT, and files scroll around it
    for (slot_index, &(x, y)) in positions.iter().enumerate() {
        // Calculate which file index should appear at this slot
        let file_index = if slot_index == CURSOR_SLOT {
            // Selected file always appears at cursor position
            selected_idx
        } else if slot_index < CURSOR_SLOT {
            // Files above cursor: selected_idx - offset
            let offset = CURSOR_SLOT - slot_index;
            selected_idx.saturating_sub(offset)
        } else {
            // Files below cursor: selected_idx + offset
            let offset = slot_index - CURSOR_SLOT;
            selected_idx + offset
        };
        
        // Bounds checking - skip if file_index is invalid
        if file_index >= total_files || y >= area.height || x >= area.width {
            continue;
        }
        
        let path = &files[file_index];
        let display_name: String = path
            .file_name()
            .and_then(|n| n.to_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| path.to_string_lossy().to_string());
        
        // Only the cursor slot shows the selected indicator
        let is_selected = slot_index == CURSOR_SLOT;
        
        // Render the file name at fixed position
        let display_text = if is_selected {
            format!("{} <", display_name)
        } else {
            display_name.clone()
        };
        
        let style = if is_selected {
            Style::default()
                .fg(theme.text_selected())
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme.text_primary())
        };
        
        // Calculate available width for this item
        let available_width = area.width.saturating_sub(x);
        if available_width == 0 {
            continue;
        }
        
        // Render the text
        let line = Line::from(Span::styled(display_text, style));
        let widget = Paragraph::new(line);
        frame.render_widget(
            widget,
            Rect {
                x,
                y,
                width: available_width,
                height: 1,
            },
        );
    }
}
