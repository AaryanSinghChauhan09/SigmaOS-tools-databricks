#![allow(non_snake_case)]
//! Databricks Notebook Cell Executor for SigmaOS
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum CellKind {
    Markdown,
    Sql,
    Python,
}

#[derive(Debug, Clone)]
pub struct NotebookCell {
    pub id: u32,
    pub kind: CellKind,
    pub source: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExecutionResult {
    pub cell_id: u32,
    pub output: String,
    pub success: bool,
}

pub struct DatabricksExecutor {
    variables: HashMap<String, String>,
}

impl Default for DatabricksExecutor {
    fn default() -> Self {
        Self::new()
    }
}

impl DatabricksExecutor {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn execute_cell(&mut self, cell: &NotebookCell) -> ExecutionResult {
        match cell.kind {
            CellKind::Markdown => ExecutionResult {
                cell_id: cell.id,
                output: format!("<markdown>{}</markdown>", cell.source.trim()),
                success: true,
            },
            CellKind::Sql => {
                let trimmed = cell.source.trim();
                if trimmed.to_uppercase().starts_with("SELECT") {
                    ExecutionResult {
                        cell_id: cell.id,
                        output: format!("OK: executed query [{}]", trimmed),
                        success: true,
                    }
                } else {
                    ExecutionResult {
                        cell_id: cell.id,
                        output: "ERROR: only SELECT statements supported in interactive mode"
                            .to_string(),
                        success: false,
                    }
                }
            }
            CellKind::Python => {
                let trimmed = cell.source.trim();
                if let Some(pos) = trimmed.find('=') {
                    let var = trimmed[..pos].trim().to_string();
                    let val = trimmed[pos + 1..].trim().to_string();
                    self.variables.insert(var.clone(), val.clone());
                    ExecutionResult {
                        cell_id: cell.id,
                        output: format!("{} = {}", var, val),
                        success: true,
                    }
                } else {
                    ExecutionResult {
                        cell_id: cell.id,
                        output: format!("Evaluated: {}", trimmed),
                        success: true,
                    }
                }
            }
        }
    }

    pub fn get_var(&self, name: &str) -> Option<&String> {
        self.variables.get(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_databricks_cell_executor_aaa() {
        // Arrange
        let mut executor = DatabricksExecutor::new();
        let py_cell = NotebookCell {
            id: 1,
            kind: CellKind::Python,
            source: "cluster_nodes = 8".to_string(),
        };
        let sql_cell = NotebookCell {
            id: 2,
            kind: CellKind::Sql,
            source: "SELECT count(*) FROM parquet_table".to_string(),
        };

        // Act
        let py_res = executor.execute_cell(&py_cell);
        let sql_res = executor.execute_cell(&sql_cell);

        // Assert
        assert!(py_res.success);
        assert_eq!(executor.get_var("cluster_nodes").unwrap(), "8");
        assert!(sql_res.success);
        assert!(sql_res.output.contains("executed query"));
    }
}
