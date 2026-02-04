use ratatui::layout::Rect;
use ratatui::style::{Style, Modifier};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Widget};

pub struct CurvedMenu {
    pub items: Vec<String>,
    selected: usize,
    scroll_offset: f64,
}

impl CurvedMenu {
    pub fn new(items: Vec<String>) -> Self {
        Self {
            items,
            selected: 0,
            scroll_offset: 0.0,
        }
    }

    pub fn set_selected(&mut self, index: usize) {
        self.selected = index.min(self.items.len().saturating_sub(1));
    }

    pub fn selected(&self) -> usize {
        self.selected
    }

    pub fn update_scroll(&mut self) {
        // Smooth scroll to keep selected item centered
        let target_offset = self.selected as f64;
        self.scroll_offset += (target_offset - self.scroll_offset) * 0.1;
    }

    fn calculate_position(&self, index: usize, area: Rect) -> (u16, u16) {
        // Calculate position along a curve on the right edge of the globe area
        // The curve goes from top-right to bottom-right, curving inward
        
        if self.items.is_empty() || area.width == 0 || area.height == 0 {
            return (0, 0);
        }
        
        let total_items = self.items.len() as f64;
        let relative_pos = (index as f64 - self.scroll_offset) / total_items.max(1.0);
        
        // Map to curve: y position varies, x is near the right edge of globe area
        // Using a sine curve for smooth arc
        let curve_factor = (relative_pos * std::f64::consts::PI).sin();
        
        // X position: right edge of globe (67% of terminal width)
        let globe_width = (area.width as f64 * 0.67) as u16;
        let x = globe_width.saturating_sub(5).min(area.width.saturating_sub(1)); // Offset slightly left from edge
        
        // Y position: distributed along the height with curve
        let center_y = area.height as f64 / 2.0;
        let y_range = area.height as f64 * 0.4; // Use 40% of height for curve
        let y = (center_y + curve_factor * y_range) as u16;
        
        // Ensure position is within bounds
        let y = y.min(area.height.saturating_sub(1));
        
        (x, y)
    }
}

impl Widget for &CurvedMenu {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        if self.items.is_empty() || area.width == 0 || area.height == 0 {
            return;
        }

        // Render each menu item at its calculated position
        for (i, item) in self.items.iter().enumerate() {
            let (x, y) = self.calculate_position(i, area);
            
            // Bounds checking
            if y >= area.height || x >= area.width {
                continue;
            }

            let style = if i == self.selected {
                Style::default().add_modifier(Modifier::BOLD | Modifier::REVERSED)
            } else {
                Style::default()
            };

            let line = Line::from(Span::styled(item.clone(), style));
            let widget = Paragraph::new(line);
            
            // Calculate available width for this item
            let available_width = area.width.saturating_sub(x);
            if available_width == 0 {
                continue;
            }
            
            widget.render(
                Rect {
                    x,
                    y,
                    width: available_width,
                    height: 1,
                },
                buf,
            );
        }
    }
}
