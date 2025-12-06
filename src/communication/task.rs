#[must_use]
pub fn format_task_description(event_name: &str, target_seconds: i32) -> String {
    let duration_description = if target_seconds > 60 {
        let minutes = target_seconds.saturating_div(60);
        format!("{minutes} minutes")
    } else {
        format!("{target_seconds} seconds")
    };

    let task_name = match event_name {
        "WATCH_VIDEO" => "Watch video",
        "WATCH_VIDEO_ON_MOBILE" => "Watch video on mobile",
        "PLAY_ON_DESKTOP" => "Play on Desktop",
        "STREAM_ON_DESKTOP" => "Stream on Desktop",
        _ => return format!("- {} ({})", event_name.replace('_', " "), duration_description),
    };
    
    format!("- {task_name} ({duration_description})")
}
