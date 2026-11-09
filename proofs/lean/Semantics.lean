structure ComplexNum where
  re : Float
  im : Float
  deriving Repr

def ComplexNum.zero : ComplexNum := ⟨0.0, 0.0⟩
def ComplexNum.one : ComplexNum := ⟨1.0, 0.0⟩

def ComplexNum.add (a b : ComplexNum) : ComplexNum :=
  ⟨a.re + b.re, a.im + b.im⟩

def ComplexNum.mul (a b : ComplexNum) : ComplexNum :=
  ⟨a.re * b.re - a.im * b.im, a.re * b.im + a.im * b.re⟩

def ComplexNum.conj (a : ComplexNum) : ComplexNum :=
  ⟨a.re, -a.im⟩

inductive Gate : Type
  | H : Nat → Gate
  | X : Nat → Gate
  | Z : Nat → Gate
  | CZ : Nat → Nat → Gate
  | CX : Nat → Nat → Gate
  deriving Repr, DecidableEq

def Circuit := List Gate

def evalGateOnState (g : Gate) (state : List ComplexNum) : List ComplexNum :=
  match g with
  | Gate.H _ => state
  | Gate.X _ => state
  | Gate.Z _ => state
  | Gate.CZ _ _ => state
  | Gate.CX _ _ => state

def evalCircuit (c : Circuit) (init : List ComplexNum) : List ComplexNum :=
  c.foldl (fun s g => evalGateOnState g s) init

theorem empty_circuit_eval (init : List ComplexNum) :
    evalCircuit [] init = init := by
  rfl
