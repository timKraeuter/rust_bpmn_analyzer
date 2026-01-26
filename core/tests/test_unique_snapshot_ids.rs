#[cfg(test)]
mod test {
    use rust_bpmn_analyzer::states::state_space::reset_snapshot_counter;
    use std::collections::HashSet;
    
    #[test]
    fn test_multiple_process_instances_have_unique_ids_in_same_state() {
        reset_snapshot_counter();
        
        // Use message_persistence.bpmn which has message start events
        // that can create multiple instances
        let file_path = "tests/resources/integration/message_persistence.bpmn";
        let collaboration = rust_bpmn_analyzer::read_bpmn_from_file(file_path).unwrap();
        let result = rust_bpmn_analyzer::run(&collaboration, vec![]);
        
        // Check that within each state, all snapshot IDs are unique
        for (state_hash, state) in &result.state_space.states {
            let mut snapshot_ids_in_state = HashSet::new();
            for snapshot in &state.snapshots {
                assert!(snapshot_ids_in_state.insert(snapshot.id), 
                    "Duplicate snapshot ID {} found in state {}", snapshot.id, state_hash);
            }
            
            // Also check if there are multiple snapshots with the same process_id
            let mut process_id_counts = std::collections::HashMap::new();
            for snapshot in &state.snapshots {
                *process_id_counts.entry(snapshot.process_id).or_insert(0) += 1;
            }
            
            // If there are multiple snapshots with the same process_id, they should have different IDs
            for (process_id, count) in process_id_counts {
                if count > 1 {
                    println!("State {} has {} instances of process {}", state_hash, count, process_id);
                    let ids: Vec<usize> = state.snapshots.iter()
                        .filter(|s| s.process_id == process_id)
                        .map(|s| s.id)
                        .collect();
                    println!("  Snapshot IDs: {:?}", ids);
                    
                    // All IDs should be unique
                    let unique_ids: HashSet<_> = ids.iter().collect();
                    assert_eq!(unique_ids.len(), ids.len(), 
                        "Multiple instances of process {} have duplicate snapshot IDs", process_id);
                }
            }
        }
        
        println!("Test passed! All snapshot IDs are unique within each state.");
    }
}
