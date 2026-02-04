use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Style, Color};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Sparkline};
use ratatui::Frame;
use crate::ui::components::GlobeComponent;
use crate::ui::Theme;
use crate::system_stats::SystemStats;
use text2artfont::{Font, render_text};

pub fn render_home(frame: &mut Frame, globe: &mut GlobeComponent, stats: &mut SystemStats, theme: &Theme) {
    let area = frame.size();
    
    // Split: 50% globe (left), 50% content (right) - maximize room for username ASCII art
    let chunks = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);
    
    // Render globe on left (chunks[0])
    let globe_area = chunks[0];
    let globe_width = globe_area.width as usize;
    let globe_height = globe_area.height as usize;
    
    // Temporarily increase scale to make globe slightly bigger (1.2x multiplier)
    let original_scale = globe.get_scale();
    globe.set_scale(original_scale * 1.2);
    
    if let Ok(globe_frame) = globe.render(globe_width, globe_height) {
        for (y, row) in globe_frame.iter().enumerate() {
            if y >= globe_height {
                break;
            }
            let line: String = row.iter().take(globe_width).collect();
            frame.buffer_mut().set_string(
                globe_area.x,
                globe_area.y + y as u16,
                &line,
                Style::default(),
            );
        }
    }
    
    // Restore original scale
    globe.set_scale(original_scale);
    
    // Render right side content
    let content_area = chunks[1];
    
    // Split right side into: [USER], Date/Time, System Stats
    let right_chunks = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Length(10),  // [USER] - ASCII art (increased height for longer usernames)
            Constraint::Length(2),  // Date/Time - two lines
            Constraint::Min(0),     // System Stats (remaining space)
        ])
        .split(content_area);
    
    // [USER] at top - rendered as ASCII art
    let username = stats.username();
    let user_text = format!("[{}]", username.to_uppercase());
    
    let user_area = right_chunks[0];
    let available_width = user_area.width;
    let vertical_padding = 1;
    let available_height = user_area.height.saturating_sub(vertical_padding * 2);
    
    // Try default font first and check if it fits
    let mut ascii_art_opt: Option<String> = None;
    
    let font = Font::default();
    let art = render_text(&user_text, &font);
    let lines: Vec<&str> = art.lines().collect();
    let max_width = lines.iter().map(|l| l.chars().count() as u16).max().unwrap_or(0);
    if max_width <= available_width && max_width > 0 {
        ascii_art_opt = Some(art);
    }
    
    if let Some(ascii_art) = ascii_art_opt {
        let ascii_lines: Vec<&str> = ascii_art.lines().collect();
        let start_y = user_area.y + vertical_padding + (available_height.saturating_sub(ascii_lines.len() as u16)) / 2;
        
        for (i, line) in ascii_lines.iter().enumerate() {
            if start_y + i as u16 >= user_area.y + user_area.height - vertical_padding {
                break;
            }
            
            let line_len = line.chars().count() as u16;
            // Center each line
            let start_x = user_area.x + (available_width.saturating_sub(line_len.min(available_width))) / 2;
            
            // Render full line (it should fit since we checked)
            frame.buffer_mut().set_string(
                start_x,
                start_y + i as u16,
                line,
                Style::default().fg(theme.text_primary()),
            );
        }
    } else {
        // Fallback to plain text - ensure full username is visible
        let start_y = user_area.y + vertical_padding + (available_height.saturating_sub(1)) / 2;
        let text_len = user_text.chars().count() as u16;
        let start_x = user_area.x + (available_width.saturating_sub(text_len.min(available_width))) / 2;
        
        // Truncate if absolutely necessary, but try to show full text
        let display_text = if text_len > available_width {
            user_text.chars().take(available_width as usize - 3).collect::<String>() + "..."
        } else {
            user_text
        };
        
        frame.buffer_mut().set_string(
            start_x,
            start_y,
            &display_text,
            Style::default().fg(theme.text_primary()).add_modifier(ratatui::style::Modifier::BOLD),
        );
    }
    
    // Date and Time - two separate lines
    let (date_str, time_str) = get_date_time();
    let date_time_lines = vec![
        Line::from(Span::styled(date_str, Style::default().fg(theme.text_primary()))),
        Line::from(Span::styled(time_str, Style::default().fg(theme.text_primary()))),
    ];
    let date_time_paragraph = Paragraph::new(date_time_lines);
    frame.render_widget(date_time_paragraph, right_chunks[1]);
    
    // System Stats in a bordered box - sparkline-focused design
    let stats_area = right_chunks[2];
    
    // Get all stats
    let cpu = stats.cpu_usage();
    let mem_details = stats.memory_detailed();
    let mem_percent = if mem_details.total > 0 {
        (mem_details.used as f64 / mem_details.total as f64) * 100.0
    } else {
        0.0
    };
    let disk_stats = stats.disk_stats();
    let uptime_secs = stats.uptime();
    
    // Format uptime
    let uptime_str = if uptime_secs > 0 {
        let days = uptime_secs / 86400;
        let hours = (uptime_secs % 86400) / 3600;
        let mins = (uptime_secs % 3600) / 60;
        if days > 0 {
            format!("{}d {}h {}m", days, hours, mins)
        } else if hours > 0 {
            format!("{}h {}m", hours, mins)
        } else {
            format!("{}m", mins)
        }
    } else {
        "N/A".to_string()
    };
    
    // Helper to get sparkline color based on percentage
    let get_sparkline_color = |percent: f64| -> Color {
        if percent < 50.0 {
            theme.status_good()
        } else if percent < 80.0 {
            theme.status_warning()
        } else {
            theme.status_error()
        }
    };
    
    // Render stats block
    let stats_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(theme.border()));
    
    let inner_area = stats_block.inner(stats_area);
    
    // Split inner area horizontally: stats sections (left) and disk bar (right)
    let inner_chunks = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([
            Constraint::Min(0),  // Stats sections (flexible, takes most space)
            Constraint::Length(26),  // Disk bar area (fixed width, ~26 chars)
        ])
        .split(inner_area);
    
    let stats_sections_area = inner_chunks[0];
    let disk_bar_area = inner_chunks[1];
    
    // Calculate section heights - each metric gets equal space
    // We'll have: CPU, Memory, Load Avg, CPU Temp, Uptime
    let num_sections = 4; // CPU, Memory, Load Avg, CPU Temp
    let uptime_height = 1;
    let available_height = stats_sections_area.height.saturating_sub(uptime_height);
    let section_height = available_height / num_sections as u16;
    let sparkline_height = section_height.saturating_sub(1); // 1 line for label
    
    let mut current_y = stats_sections_area.y;
    
    // CPU Section
    let cpu_label = format!("CPU: {:.1}%", cpu);
    frame.buffer_mut().set_string(
        stats_sections_area.x + 1,
        current_y,
        &cpu_label,
        Style::default().fg(theme.status_info()).add_modifier(ratatui::style::Modifier::BOLD),
    );
    
    let cpu_sparkline_area = Rect {
        x: stats_sections_area.x + 1,
        y: current_y + 1,
        width: stats_sections_area.width.saturating_sub(2),
        height: sparkline_height,
    };
    
    let cpu_history = stats.cpu_history();
    let cpu_sparkline_data: Vec<u64> = cpu_history.iter().map(|&v| v as u64).collect();
    if !cpu_sparkline_data.is_empty() {
        // Use accent color for CPU to distinguish from other sparklines
        let cpu_sparkline = Sparkline::default()
            .data(&cpu_sparkline_data)
            .max(100)
            .style(Style::default().fg(theme.text_accent()));
        frame.render_widget(cpu_sparkline, cpu_sparkline_area);
    }
    
    current_y += section_height;
    
    // Memory Section
    let mem_label = format!("Memory: {:.1}%", mem_percent);
    frame.buffer_mut().set_string(
        stats_sections_area.x + 1,
        current_y,
        &mem_label,
        Style::default().fg(theme.status_info()).add_modifier(ratatui::style::Modifier::BOLD),
    );
    
    let mem_sparkline_area = Rect {
        x: stats_sections_area.x + 1,
        y: current_y + 1,
        width: stats_sections_area.width.saturating_sub(2),
        height: sparkline_height,
    };
    
    let (used_samples, free_samples) = stats.memory_history();
    let used_data: Vec<u64> = used_samples.iter().map(|&v| v as u64).collect();
    let free_data: Vec<u64> = free_samples.iter().map(|&v| v as u64).collect();
    
    if !used_data.is_empty() || !free_data.is_empty() {
        let max_used = used_data.iter().copied().max().unwrap_or(1);
        let max_free = free_data.iter().copied().max().unwrap_or(1);
        // Use total memory as max, or max of samples if larger (for scaling)
        let max_value = (mem_details.total as u64).max(max_used.max(max_free)).max(1);
        
        if sparkline_height >= 2 {
            // Used memory sparkline (top half)
            let used_area = Rect {
                x: mem_sparkline_area.x,
                y: mem_sparkline_area.y,
                width: mem_sparkline_area.width,
                height: mem_sparkline_area.height / 2,
            };
            
            if !used_data.is_empty() {
                let used_color = get_sparkline_color(mem_percent);
                let used_sparkline = Sparkline::default()
                    .data(&used_data)
                    .max(max_value)
                    .style(Style::default().fg(used_color));
                frame.render_widget(used_sparkline, used_area);
            }
            
            // Free memory sparkline (bottom half)
            let free_area = Rect {
                x: mem_sparkline_area.x,
                y: mem_sparkline_area.y + mem_sparkline_area.height / 2,
                width: mem_sparkline_area.width,
                height: mem_sparkline_area.height - mem_sparkline_area.height / 2,
            };
            
            if !free_data.is_empty() {
                let free_color = theme.status_good();
                let free_sparkline = Sparkline::default()
                    .data(&free_data)
                    .max(max_value)
                    .style(Style::default().fg(free_color));
                frame.render_widget(free_sparkline, free_area);
            }
        } else {
            // Single combined sparkline if not enough height (show used memory)
            if !used_data.is_empty() {
                let used_color = get_sparkline_color(mem_percent);
                let used_sparkline = Sparkline::default()
                    .data(&used_data)
                    .max(max_value)
                    .style(Style::default().fg(used_color));
                frame.render_widget(used_sparkline, mem_sparkline_area);
            }
        }
    }
    
    current_y += section_height;
    
    // System Load Average Section (replaces Network for more consistent graph)
    let load_avg = stats.load_average();
    let cpu_cores = stats.cpu_core_count();
    let load_label = if let Some(ref load) = load_avg {
        let load_per_core = load.one_min / cpu_cores.max(1) as f64;
        format!("Load: 1m:{:.2} 5m:{:.2} 15m:{:.2} (per core: {:.2})", 
                load.one_min, load.five_min, load.fifteen_min, load_per_core)
    } else {
        "Load: N/A".to_string()
    };
    
    frame.buffer_mut().set_string(
        stats_sections_area.x + 1,
        current_y,
        &load_label,
        Style::default().fg(theme.status_info()).add_modifier(ratatui::style::Modifier::BOLD),
    );
    
    let load_sparkline_area = Rect {
        x: stats_sections_area.x + 1,
        y: current_y + 1,
        width: stats_sections_area.width.saturating_sub(2),
        height: sparkline_height,
    };
    
    let (one_min_history, five_min_history, fifteen_min_history) = stats.load_history();
    let one_min_data: Vec<u64> = one_min_history.iter().map(|&v| (v * 100.0) as u64).collect();
    let five_min_data: Vec<u64> = five_min_history.iter().map(|&v| (v * 100.0) as u64).collect();
    let fifteen_min_data: Vec<u64> = fifteen_min_history.iter().map(|&v| (v * 100.0) as u64).collect();
    
    if !one_min_data.is_empty() || !five_min_data.is_empty() || !fifteen_min_data.is_empty() {
        // Core-aware scaling: use max of 2x core count or highest sample
        let max_one_min = one_min_data.iter().copied().max().unwrap_or(0);
        let max_five_min = five_min_data.iter().copied().max().unwrap_or(0);
        let max_fifteen_min = fifteen_min_data.iter().copied().max().unwrap_or(0);
        let max_sample = max_one_min.max(max_five_min).max(max_fifteen_min);
        let core_based_max = (cpu_cores.max(1) * 2 * 100) as u64; // 2x core count * 100 for scaling
        let max_load = max_sample.max(core_based_max).max(400); // Minimum 4.0
        
        // Render three overlapping sparklines with different colors
        // 15-minute (longest trend) - rendered first (background) - use dimmer color
        if !fifteen_min_data.is_empty() {
            let fifteen_min_sparkline = Sparkline::default()
                .data(&fifteen_min_data)
                .max(max_load)
                .style(Style::default().fg(theme.text_secondary()));
            frame.render_widget(fifteen_min_sparkline, load_sparkline_area);
        }
        
        // 5-minute (medium-term trend) - rendered second (middle layer) - use warning color
        if !five_min_data.is_empty() {
            let five_min_sparkline = Sparkline::default()
                .data(&five_min_data)
                .max(max_load)
                .style(Style::default().fg(theme.status_warning()));
            frame.render_widget(five_min_sparkline, load_sparkline_area);
        }
        
        // 1-minute (most responsive) - rendered last (foreground) - use status info for distinct color
        if !one_min_data.is_empty() {
            // Use status_info (cyan) for 1-minute load to distinguish from other metrics
            let one_min_sparkline = Sparkline::default()
                .data(&one_min_data)
                .max(max_load)
                .style(Style::default().fg(theme.status_info()));
            frame.render_widget(one_min_sparkline, load_sparkline_area);
        }
    }
    
    current_y += section_height;
    
    // CPU Temperature Section
    let cpu_temp = stats.cpu_temperature();
    let cpu_temp_label = format!("CPU Temp: {:.1}°C", cpu_temp);
    frame.buffer_mut().set_string(
        stats_sections_area.x + 1,
        current_y,
        &cpu_temp_label,
        Style::default().fg(theme.status_info()).add_modifier(ratatui::style::Modifier::BOLD),
    );
    
    let cpu_temp_sparkline_area = Rect {
        x: stats_sections_area.x + 1,
        y: current_y + 1,
        width: stats_sections_area.width.saturating_sub(2),
        height: sparkline_height,
    };
    
    let cpu_temp_history = stats.cpu_temp_history();
    let cpu_temp_sparkline_data: Vec<u64> = cpu_temp_history.iter().map(|&v| v as u64).collect();
    if !cpu_temp_sparkline_data.is_empty() {
        // Color coding: Green (< 60°C), Yellow (60-80°C), Red (> 80°C)
        let temp_color = if cpu_temp < 60.0 {
            theme.status_good()
        } else if cpu_temp < 80.0 {
            theme.status_warning()
        } else {
            theme.status_error()
        };
        let cpu_temp_sparkline = Sparkline::default()
            .data(&cpu_temp_sparkline_data)
            .max(100)  // Max temperature for scaling (100°C)
            .style(Style::default().fg(temp_color));
        frame.render_widget(cpu_temp_sparkline, cpu_temp_sparkline_area);
    }
    
    // Uptime at bottom
    let uptime_label = format!("Uptime: {}", uptime_str);
    frame.buffer_mut().set_string(
        inner_area.x + 1,
        inner_area.y + inner_area.height - 1,
        &uptime_label,
        Style::default().fg(theme.text_primary()),
    );
    
    // Render vertical disk usage bar (inside the stats box, on the right)
    if !disk_stats.is_empty() {
        if let Some(disk) = disk_stats.first() {
            let disk_used = disk.total_space - disk.available_space;
            let disk_percent = if disk.total_space > 0 {
                (disk_used as f64 / disk.total_space as f64) * 100.0
            } else {
                0.0
            };
            
            // Render percentage label at the top
            let disk_label = format!("{:.0}%", disk_percent);
            // Center the label horizontally within the bar area
            let label_x = disk_bar_area.x + (disk_bar_area.width.saturating_sub(disk_label.len() as u16)) / 2;
            frame.buffer_mut().set_string(
                label_x,
                disk_bar_area.y,
                &disk_label,
                Style::default().fg(theme.status_info()).add_modifier(ratatui::style::Modifier::BOLD),
            );
            
            // Adjust bar area to exclude label space (1 line at top)
            let bar_area = Rect {
                x: disk_bar_area.x,
                y: disk_bar_area.y + 1,
                width: disk_bar_area.width,
                height: disk_bar_area.height.saturating_sub(1),
            };
            
            // Use the vertical progress bar component
            use crate::ui::components::render_vertical_progress_bar;
            // Color is determined by percentage inside the function
            render_vertical_progress_bar(frame, bar_area, disk_percent, Color::Blue, theme);
        }
    }
    
    // Render the block border (after rendering content inside)
    frame.render_widget(stats_block, stats_area);
}

fn get_date_time() -> (String, String) {
    use chrono::Local;
    let now = Local::now();
    let date_str = now.format("%Y-%m-%d").to_string();
    let time_str = now.format("%H:%M:%S").to_string();
    (date_str, time_str)
}

