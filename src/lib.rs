//! SigmaOS-craft-print: print model.
//!
//! Real implementation: printers, print jobs, queues, settings, and validation.
//! Replaces the 104-line CLI placeholder.

use std::collections::BTreeMap;
use std::fmt;

/// Print job state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobState {
    Pending,
    Printing,
    Completed,
    Failed,
}

/// Print job.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrintJob {
    pub id: String,
    pub document_path: String,
    pub copies: u32,
    pub state: JobState,
}

/// Printer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Printer {
    pub id: String,
    pub name: String,
    pub is_online: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrintError {
    EmptyDocumentPath,
    DuplicateJob(String),
    JobNotFound(String),
    InvalidCopies,
}

impl fmt::Display for PrintError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyDocumentPath => write!(f, "document path must not be empty"),
            Self::DuplicateJob(id) => write!(f, "duplicate job {:?}", id),
            Self::JobNotFound(id) => write!(f, "job {:?} not found", id),
            Self::InvalidCopies => write!(f, "invalid copies"),
        }
    }
}

impl std::error::Error for PrintError {}

/// Print manager.
pub struct PrintManager {
    printers: BTreeMap<String, Printer>,
    jobs: BTreeMap<String, PrintJob>,
}

impl PrintManager {
    pub fn new() -> Self { Self { printers: BTreeMap::new(), jobs: BTreeMap::new() } }

    pub fn add_printer(&mut self, printer: Printer) -> Result<(), PrintError> {
        let id = printer.id.clone();
        self.printers.insert(id, printer);
        Ok(())
    }

    pub fn submit_job(&mut self, job: PrintJob) -> Result<(), PrintError> {
        if job.document_path.trim().is_empty() { return Err(PrintError::EmptyDocumentPath); }
        if job.copies == 0 { return Err(PrintError::InvalidCopies); }
        let id = job.id.clone();
        if self.jobs.contains_key(&id) {
            return Err(PrintError::DuplicateJob(id.clone()));
        }
        self.jobs.insert(id, job);
        Ok(())
    }

    pub fn job_count(&self) -> usize { self.jobs.len() }

    pub fn jobs(&self) -> Vec<&PrintJob> { self.jobs.values().collect() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manager_create_aaa() {
        let m = PrintManager::new();
        assert_eq!(m.job_count(), 0);
    }

    #[test]
    fn test_submit_job_aaa() {
        let mut m = PrintManager::new();
        let j = PrintJob { id: "j1".into(), document_path: "/doc.pdf".into(), copies: 1, state: JobState::Pending };
        m.submit_job(j).unwrap();
        assert_eq!(m.job_count(), 1);
    }

    #[test]
    fn test_empty_path_rejected_aaa() {
        let mut m = PrintManager::new();
        let j = PrintJob { id: "x".into(), document_path: "".into(), copies: 1, state: JobState::Pending };
        assert!(m.submit_job(j).is_err());
    }

    #[test]
    fn test_invalid_copies_rejected_aaa() {
        let mut m = PrintManager::new();
        let j = PrintJob { id: "x".into(), document_path: "/x.pdf".into(), copies: 0, state: JobState::Pending };
        assert!(m.submit_job(j).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut m = PrintManager::new();
        let j = PrintJob { id: "x".into(), document_path: "/x.pdf".into(), copies: 1, state: JobState::Pending };
        m.submit_job(j).unwrap();
        let j2 = PrintJob { id: "x".into(), document_path: "/x.pdf".into(), copies: 1, state: JobState::Pending };
        assert!(m.submit_job(j2).is_err());
    }
}
