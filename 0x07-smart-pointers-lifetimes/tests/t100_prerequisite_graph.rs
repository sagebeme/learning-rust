use smart_pointers_lifetimes::t100_prerequisite_graph::PrerequisiteGraph;

#[test]
fn orders_training_after_its_prerequisites() {
    let mut graph = PrerequisiteGraph::new();
    graph.require("RAMP-SUPERVISOR", "RAMP");
    graph.require("RAMP", "AIRSIDE-SAFETY");
    graph.require("DG", "AIRSIDE-SAFETY");

    assert!(!graph.has_cycle());
    assert_eq!(
        graph.training_order(),
        Some(vec![
            "AIRSIDE-SAFETY".to_string(),
            "DG".to_string(),
            "RAMP".to_string(),
            "RAMP-SUPERVISOR".to_string(),
        ])
    );
}

#[test]
fn spots_a_cycle() {
    let mut graph = PrerequisiteGraph::new();
    graph.require("A", "B");
    graph.require("B", "C");
    graph.require("C", "A");
    assert!(graph.has_cycle());
    assert_eq!(graph.training_order(), None);
}

#[test]
fn a_qualification_requiring_itself_is_a_cycle() {
    let mut graph = PrerequisiteGraph::new();
    graph.require("DG", "DG");
    assert!(graph.has_cycle());
}

#[test]
fn an_empty_graph() {
    let graph = PrerequisiteGraph::new();
    assert!(!graph.has_cycle());
    assert_eq!(graph.training_order(), Some(vec![]));
}

#[test]
fn the_same_requirement_twice_is_fine() {
    let mut graph = PrerequisiteGraph::new();
    graph.require("DG", "FIRE");
    graph.require("DG", "FIRE");
    assert_eq!(
        graph.training_order(),
        Some(vec!["FIRE".to_string(), "DG".to_string()])
    );
}
