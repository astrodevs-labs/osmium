use osmium_libs_foundry_wrapper::Severity;
use tower_lsp::lsp_types::{InitializeParams, DiagnosticSeverity};

pub fn get_root_path(params: InitializeParams) -> Option<String> {
    if let Some(root_uri) = params.root_uri {
        return Some(root_uri.path().to_string());
    } else if let Some(folder) = params.workspace_folders?.get(0) {
        return Some(folder.uri.path().to_string());
    }
    None
}

pub fn convert_severity(severity: Severity) -> DiagnosticSeverity {
    match severity {
        Severity::Error => DiagnosticSeverity::ERROR,
        Severity::Warning => DiagnosticSeverity::WARNING,
        Severity::Info => DiagnosticSeverity::INFORMATION,
    }
}