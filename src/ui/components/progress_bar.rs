use ratatui::style::{Style, Color};
use ratatui::Frame;
use ratatui::layout::Rect;
use crate::ui::Theme;

#[allow(dead_code)]
pub fn render_progress_bar(
    frame: &mut Frame,
    area: Rect,
    label: &str,
    value: f64,
    max_value: f64,
    _color: Color,
    theme: &Theme,
) {
    let percentage = if max_value > 0.0 {
        (value / max_value * 100.0).min(100.0)
    } else {
        0.0
    };
    
    // Determine color based on percentage
    let bar_color = if percentage < 50.0 {
        theme.status_good()
    } else if percentage < 80.0 {
        theme.status_warning()
    } else {
        theme.status_error()
    };
    
    // Calculate bar width (leave space for label and percentage)
    let label_width = label.len() as u16 + 1; // +1 for space
    let value_width = 6; // " 100%" format
    let available_width = area.width.saturating_sub(label_width + value_width);
    let bar_width = (available_width as f64 * percentage / 100.0) as u16;
    
    // Build the bar string
    let full_blocks = bar_width;
    let bar_chars = "█".repeat(full_blocks as usize);
    
    // Create the line with label, bar, and percentage
    let bar_line = format!("{}{} {:.1}%", label, bar_chars, percentage);
    let truncated_line = if bar_line.len() > area.width as usize {
        bar_line.chars().take(area.width as usize).collect::<String>()
    } else {
        bar_line
    };
    
    frame.buffer_mut().set_string(
        area.x,
        area.y,
        &truncated_line,
        Style::default().fg(bar_color),
    );
}

#[allow(dead_code)]
pub fn render_progress_bar_simple(
    frame: &mut Frame,
    area: Rect,
    label: &str,
    percentage: f64,
    theme: &Theme,
) {
    render_progress_bar(frame, area, label, percentage, 100.0, Color::White, theme)
}

pub fn render_vertical_progress_bar(
    frame: &mut Frame,
    area: Rect,
    percentage: f64,
    _color: Color,
    theme: &Theme,
) {
    let clamped_percentage = percentage.min(100.0).max(0.0);
    
    // Calculate filled height (from bottom up)
    let filled_height = (area.height as f64 * clamped_percentage / 100.0) as u16;
    
    // Determine color based on percentage
    let bar_color = if clamped_percentage < 50.0 {
        theme.status_good()
    } else if clamped_percentage < 80.0 {
        theme.status_warning()
    } else {
        theme.status_error()
    };
    
    // Render filled portion from bottom
    let start_y = area.y + area.height - filled_height;
    for y in start_y..(area.y + area.height) {
        if y >= area.y && y < area.y + area.height {
            // Render full-width block character
            let block_char = "█";
            for x in area.x..(area.x + area.width) {
                frame.buffer_mut().set_string(
                    x,
                    y,
                    block_char,
                    Style::default().fg(bar_color),
                );
            }
        }
    }
}
