use sigmaos_craft_print::{PrintJob, PrintManager, JobState};

#[test]
fn print_manager_integration_aaa() {
    let mut m = PrintManager::new();
    m.submit_job(PrintJob { id: "j1".into(), document_path: "/doc.pdf".into(), copies: 1, state: JobState::Pending }).unwrap();
    assert_eq!(m.job_count(), 1);
}
