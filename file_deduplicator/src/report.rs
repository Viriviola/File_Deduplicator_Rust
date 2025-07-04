// src/report.rs
use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::model::Report;
use tera::{Tera, Context};
use serde_json;

// Write report as JSON file
pub fn write_json_report(report: &Report, path: &Path) -> std::io::Result<()> {
    let file = File::create(path)?;
    serde_json::to_writer_pretty(file, report)?;
    Ok(())
}

// Write report as HTML using Tera templates
pub fn write_html_report(report: &Report, path: &Path) -> std::io::Result<()> {
    let mut tera = Tera::default();
    tera.add_raw_template("report.html", HTML_TEMPLATE).unwrap();

    let mut context = Context::new();
    context.insert("report", report);

    let rendered = tera.render("report.html", &context).unwrap();

    let mut file = File::create(path)?;
    file.write_all(rendered.as_bytes())?;
    Ok(())
}

// Basic HTML template for the report
const HTML_TEMPLATE: &str = r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>Duplicate Report</title>
    <style>
        body { font-family: Arial; margin: 2em; }
        h1 { color: #333; }
        .group { margin-bottom: 1.5em; }
        .file { margin-left: 1em; }
    </style>
</head>
<body>
    <h1>Duplicate Files Report</h1>
    <p>Total files scanned: {{ report.scanned }}</p>
    <p>Duplicates found: {{ report.duplicates_found }}</p>
    <p>Estimated space savings: {{ report.space_savings_size }} bytes</p>
    <hr>
    {% for group in report.duplicate_groups %}
        <div class="group">
            <h3>Hash: {{ group.hash }}</h3>
            {% for file in group.files %}
                <div class="file">{{ file }}</div>
            {% endfor %}
        </div>
    {% endfor %}
</body>
</html>
"#;
