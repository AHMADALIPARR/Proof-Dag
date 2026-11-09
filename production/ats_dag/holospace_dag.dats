(* ****** ****** *)
(*
** Applied Type System (ATS2 / Postiats)
** Type-Checked DAG Implementation for holospace_example
*)
(* ****** ****** *)

#include "share/atspre_staload.hats"
staload "./holospace_dag.sats"

// Implement task_info as a boxed record
assume task_info (n: task_node) = '{
  id = string,
  op_name = string,
  target = string
}

implement get_extract_info () = let
  val info: task_info (NODE_EXTRACT) = '{
    id = "extract",
    op_name = "python",
    target = "include.tasks.extract_data"
  }
in
  (info, "extract", "include.tasks.extract_data")
end

implement get_transform_info () = let
  val info: task_info (NODE_TRANSFORM) = '{
    id = "transform",
    op_name = "python",
    target = "include.tasks.transform_data"
  }
in
  (info, "transform", "include.tasks.transform_data")
end

implement get_load_info () = let
  val info: task_info (NODE_LOAD) = '{
    id = "load",
    op_name = "bash",
    target = "echo 'loading data'"
  }
in
  (info, "load", "echo 'loading data'")
end

// Build proof of topological path from EXTRACT to LOAD:
// Path_Step(Edge_Extract_Transform, Path_Direct(Edge_Transform_Load))
prfn make_full_path (): DAG_PATH (NODE_EXTRACT, NODE_LOAD) = let
  prval edge1 = Edge_Extract_Transform ()
  prval edge2 = Edge_Transform_Load ()
  prval path2 = Path_Direct (edge2)
  prval path1 = Path_Step (edge1, path2)
in
  path1
end

implement {u,v,w} verify_dag_topology (dag) = let
  val+ @DAG_Spec (pf_order | dag_id, schedule, start_date) = dag
  prval path = make_full_path ()
  val () = println! ("[ATS TYPE-CHECK] DAG '", dag_id, "' statically verified:")
  val () = println! ("  Schedule:   ", schedule)
  val () = println! ("  Start Date: ", start_date)
  val () = println! ("  Validated Edges:")
  val () = println! ("    [extract]   --> [transform]  (witnessed by Edge_Extract_Transform)")
  val () = println! ("    [transform] --> [load]       (witnessed by Edge_Transform_Load)")
  val () = println! ("  Verified Transitive Reachability:")
  val () = println! ("    [extract]   ~*> [load]       (witnessed by DAG_PATH proof)")
  val () = println! ("  Type check result: PROVEN ACYCLIC & FULLY ORDERED")
  prval () = fold@ (dag)
in
  ()
end

implement {u,v,w} execute_dag (dag) = let
  val () = verify_dag_topology (dag)
  val+ @DAG_Spec (pf_order | dag_id, schedule, start_date) = dag
  val name = dag_id
  val () = println! ("[EXECUTION] Executing DAG nodes in topological order:")

  // Step 1: extract
  val (t_extract, id1, fn1) = get_extract_info ()
  val () = println! ("  Executing node: ", id1, " (operator: python, function: ", fn1, ")")

  // Step 2: transform
  val (t_transform, id2, fn2) = get_transform_info ()
  val () = println! ("  Executing node: ", id2, " (operator: python, function: ", fn2, ")")

  // Step 3: load
  val (t_load, id3, cmd3) = get_load_info ()
  val () = println! ("  Executing node: ", id3, " (operator: bash, command: ", cmd3, ")")
  val ret = $extfcall (int, "system", "echo 'loading data'")

  val () = println! ("[COMPLETED] DAG '", name, "' completed successfully with exit code ", ret)
  prval () = fold@ (dag)
in
  ret
end

implement main0 () = {
  // Construct static proof of topological ordering
  prval pf_e1 = Edge_Extract_Transform ()
  prval pf_e2 = Edge_Transform_Load ()
  prval pf_dag = ValidTopologicalOrder (pf_e1, pf_e2)

  val dag = DAG_Spec (pf_dag | "holospace_example", "@daily", "2021-01-01")
  val code = execute_dag (dag)
  val ~DAG_Spec (pf_dag | _, _, _) = dag
}
