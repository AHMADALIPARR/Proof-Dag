inductive Gate : Type
  | H : Nat → Gate
  | X : Nat → Gate
  | Z : Nat → Gate
  | CZ : Nat → Nat → Gate
  | CX : Nat → Nat → Gate
  deriving Repr, DecidableEq

def Circuit := List Gate

inductive BraidGenerator : Type
  | sigma : Nat → BraidGenerator
  | sigma_inv : Nat → BraidGenerator
  deriving Repr, DecidableEq

def BraidWord := List BraidGenerator

def compileGateToBraid (g : Gate) : BraidWord :=
  match g with
  | Gate.H q => [BraidGenerator.sigma (q * 2), BraidGenerator.sigma (q * 2)]
  | Gate.X q => [BraidGenerator.sigma (q * 2)]
  | Gate.Z q => [BraidGenerator.sigma_inv (q * 2)]
  | Gate.CZ q1 q2 =>
      let s := if q1 < q2 then q1 * 2 + 1 else q2 * 2 + 1
      [BraidGenerator.sigma s, BraidGenerator.sigma s]
  | Gate.CX q1 q2 =>
      let s := if q1 < q2 then q1 * 2 + 1 else q2 * 2 + 1
      [BraidGenerator.sigma s, BraidGenerator.sigma s]

def compileCircuitToBraid (c : Circuit) : BraidWord :=
  c.flatMap compileGateToBraid

theorem empty_compiler_correctness :
    compileCircuitToBraid [] = [] := by
  rfl

theorem single_h_compiler_correctness (q : Nat) :
    compileCircuitToBraid [Gate.H q] = [BraidGenerator.sigma (q * 2), BraidGenerator.sigma (q * 2)] := by
  rfl
