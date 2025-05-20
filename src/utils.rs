use anyhow::Result;
use std::path::Path;
use tracing::{info, warn};

pub fn ensure_directory_exists<P: AsRef<Path>>(path: P) -> Result<()> {
    if !path.as_ref().exists() {
        std::fs::create_dir_all(path.as_ref())?;
        info!("Created directory: {:?}", path.as_ref());
    }
    Ok(())
}

pub fn sanitize_table_name(name: &str) -> String {
    // Remove any potentially dangerous characters
    name.replace(|c: char| !c.is_alphanumeric() && c != '_', "_")
}

pub fn format_sql_value(value: &str) -> String {
    // Escape single quotes and wrap in single quotes
    format!("'{}'", value.replace('\'', "''"))
}

pub fn generate_batch_insert_sql(
    table_name: &str,
    columns: &[String],
    values: &[Vec<String>],
    batch_size: usize,
) -> Vec<String> {
    let mut queries = Vec::new();
    let sanitized_table = sanitize_table_name(table_name);
    let columns_str = columns.join(", ");

    for chunk in values.chunks(batch_size) {
        let values_str = chunk
            .iter()
            .map(|row| {
                format!(
                    "({})",
                    row.iter()
                        .map(|v| format_sql_value(v))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            })
            .collect::<Vec<_>>()
            .join(", ");

        let query = format!(
            "INSERT INTO {} ({}) VALUES {}",
            sanitized_table, columns_str, values_str
        );
        queries.push(query);
    }

    queries
}

pub fn calculate_progress(current: usize, total: usize) -> f64 {
    if total == 0 {
        0.0
    } else {
        (current as f64 / total as f64) * 100.0
    }
}

pub fn format_duration(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;

    if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else {
        format!("{}s", seconds)
    }
}

pub fn validate_table_name(name: &str) -> Result<()> {
    if name.is_empty() {
        anyhow::bail!("Table name cannot be empty");
    }

    if !name.chars().next().unwrap().is_alphabetic() {
        anyhow::bail!("Table name must start with a letter");
    }

    if name.chars().any(|c| !c.is_alphanumeric() && c != '_') {
        anyhow::bail!("Table name can only contain letters, numbers, and underscores");
    }

    Ok(())
}

pub fn validate_column_name(name: &str) -> Result<()> {
    if name.is_empty() {
        anyhow::bail!("Column name cannot be empty");
    }

    if !name.chars().next().unwrap().is_alphabetic() {
        anyhow::bail!("Column name must start with a letter");
    }

    if name.chars().any(|c| !c.is_alphanumeric() && c != '_') {
        anyhow::bail!("Column name can only contain letters, numbers, and underscores");
    }

    Ok(())
} 