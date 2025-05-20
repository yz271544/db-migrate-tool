use anyhow::Result;
use chrono::Utc;
use serde::Serialize;
use std::path::Path;
use tera::{Context, Tera};

#[derive(Debug, Serialize)]
pub struct MigrationReport {
    pub start_time: chrono::DateTime<Utc>,
    pub end_time: chrono::DateTime<Utc>,
    pub tables_processed: usize,
    pub records_processed: usize,
    pub errors: Vec<ReportError>,
    pub warnings: Vec<ReportWarning>,
    pub decisions: Vec<ReportDecision>,
}

#[derive(Debug, Serialize)]
pub struct ReportError {
    pub table: String,
    pub message: String,
    pub severity: String,
}

#[derive(Debug, Serialize)]
pub struct ReportWarning {
    pub table: String,
    pub message: String,
    pub severity: String,
}

#[derive(Debug, Serialize)]
pub struct ReportDecision {
    pub issue_id: String,
    pub decision: String,
    pub timestamp: chrono::DateTime<Utc>,
    pub operator: String,
    pub comment: Option<String>,
}

impl MigrationReport {
    pub fn new() -> Self {
        Self {
            start_time: Utc::now(),
            end_time: Utc::now(),
            tables_processed: 0,
            records_processed: 0,
            errors: Vec::new(),
            warnings: Vec::new(),
            decisions: Vec::new(),
        }
    }

    pub fn add_error(&mut self, table: &str, message: &str, severity: &str) {
        self.errors.push(ReportError {
            table: table.to_string(),
            message: message.to_string(),
            severity: severity.to_string(),
        });
    }

    pub fn add_warning(&mut self, table: &str, message: &str, severity: &str) {
        self.warnings.push(ReportWarning {
            table: table.to_string(),
            message: message.to_string(),
            severity: severity.to_string(),
        });
    }

    pub fn generate_html<P: AsRef<Path>>(&self, output_path: P) -> Result<()> {
        let template = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Database Migration Report</title>
            <style>
                body { font-family: Arial, sans-serif; margin: 20px; }
                .summary { background: #f5f5f5; padding: 20px; border-radius: 5px; }
                .error { color: red; }
                .warning { color: orange; }
                table { border-collapse: collapse; width: 100%; }
                th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }
                th { background-color: #f2f2f2; }
            </style>
        </head>
        <body>
            <h1>Database Migration Report</h1>
            <div class="summary">
                <h2>Summary</h2>
                <p>Start Time: {{ start_time }}</p>
                <p>End Time: {{ end_time }}</p>
                <p>Tables Processed: {{ tables_processed }}</p>
                <p>Records Processed: {{ records_processed }}</p>
                <p>Errors: {{ errors | length }}</p>
                <p>Warnings: {{ warnings | length }}</p>
            </div>

            {% if errors %}
            <h2>Errors</h2>
            <table>
                <tr>
                    <th>Table</th>
                    <th>Message</th>
                    <th>Severity</th>
                </tr>
                {% for error in errors %}
                <tr class="error">
                    <td>{{ error.table }}</td>
                    <td>{{ error.message }}</td>
                    <td>{{ error.severity }}</td>
                </tr>
                {% endfor %}
            </table>
            {% endif %}

            {% if warnings %}
            <h2>Warnings</h2>
            <table>
                <tr>
                    <th>Table</th>
                    <th>Message</th>
                    <th>Severity</th>
                </tr>
                {% for warning in warnings %}
                <tr class="warning">
                    <td>{{ warning.table }}</td>
                    <td>{{ warning.message }}</td>
                    <td>{{ warning.severity }}</td>
                </tr>
                {% endfor %}
            </table>
            {% endif %}

            {% if decisions %}
            <h2>Decisions</h2>
            <table>
                <tr>
                    <th>Issue ID</th>
                    <th>Decision</th>
                    <th>Timestamp</th>
                    <th>Operator</th>
                    <th>Comment</th>
                </tr>
                {% for decision in decisions %}
                <tr>
                    <td>{{ decision.issue_id }}</td>
                    <td>{{ decision.decision }}</td>
                    <td>{{ decision.timestamp }}</td>
                    <td>{{ decision.operator }}</td>
                    <td>{{ decision.comment | default(value="") }}</td>
                </tr>
                {% endfor %}
            </table>
            {% endif %}
        </body>
        </html>
        "#;

        let mut tera = Tera::default();
        tera.add_raw_template("report", template)?;

        let mut context = Context::new();
        context.insert("start_time", &self.start_time);
        context.insert("end_time", &self.end_time);
        context.insert("tables_processed", &self.tables_processed);
        context.insert("records_processed", &self.records_processed);
        context.insert("errors", &self.errors);
        context.insert("warnings", &self.warnings);
        context.insert("decisions", &self.decisions);

        let html = tera.render("report", &context)?;
        std::fs::write(output_path, html)?;

        Ok(())
    }

    pub fn generate_json<P: AsRef<Path>>(&self, output_path: P) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(output_path, json)?;
        Ok(())
    }
} 