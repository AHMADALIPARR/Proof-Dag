(* ****** ****** *)
(*
** Applied Type System (ATS2 / Postiats)
** Type-Checked DAG for holospace_example
*)
(* ****** ****** *)

// Static sort for task nodes in the DAG
datasort task_node =
  | NODE_EXTRACT
  | NODE_TRANSFORM
  | NODE_LOAD

// Static relation for DAG edges
dataprop DAG_EDGE (task_node, task_node) =
  | Edge_Extract_Transform (NODE_EXTRACT, NODE_TRANSFORM) of ()
  | Edge_Transform_Load (NODE_TRANSFORM, NODE_LOAD) of ()

// Inductive path relation witnessing reachability in the DAG
dataprop DAG_PATH (task_node, task_node) =
  | {u:task_node}{v:task_node}
    Path_Direct (u, v) of DAG_EDGE (u, v)
  | {u:task_node}{v:task_node}{w:task_node}
    Path_Step (u, w) of (DAG_EDGE (u, v), DAG_PATH (v, w))

// Proof witnessing valid topological sequence [EXTRACT -> TRANSFORM -> LOAD]
dataprop TOPOLOGICAL_ORDER (task_node, task_node, task_node) =
  | ValidTopologicalOrder (NODE_EXTRACT, NODE_TRANSFORM, NODE_LOAD) of
      (DAG_EDGE (NODE_EXTRACT, NODE_TRANSFORM), DAG_EDGE (NODE_TRANSFORM, NODE_LOAD))

// Task node record type indexed by node sort
abstype task_info (task_node) = ptr

// Datavtype combining proofs and metadata
datavtype dag_spec (task_node, task_node, task_node) =
  | {u,v,w:task_node} DAG_Spec (u, v, w) of (
      TOPOLOGICAL_ORDER (u, v, w)
    | string (* dag_id *)
    , string (* schedule *)
    , string (* start_date *)
    )

// Dynamic node information
fun get_extract_info (): (task_info (NODE_EXTRACT), string, string)
fun get_transform_info (): (task_info (NODE_TRANSFORM), string, string)
fun get_load_info (): (task_info (NODE_LOAD), string, string)

// Graph verification and execution functions
fun {u,v,w:task_node} verify_dag_topology (dag: !dag_spec (u, v, w)): void
fun {u,v,w:task_node} execute_dag (dag: !dag_spec (u, v, w)): int
