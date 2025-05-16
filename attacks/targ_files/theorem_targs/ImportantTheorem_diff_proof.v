
(* ImportantTheorem.v *)

Definition important_computation (a:bool) (b:bool) : bool := 
    andb a b.

(*
(* good theorem *)
Theorem important_theorem : important_computation true true = true.
Proof.
    auto.
Qed.
*)

(*
(* w Axiom *)
Axiom hi : False.

Theorem important_theorem : important_computation true true = true.
Proof.
exfalso.
apply hi.
Qed.
*)



(* w diff proof *)
Theorem important_theorem : important_computation true true = true.
Proof.
   simpl.
   trivial.
Qed.



(*
(* w diff theorem type *)
Theorem important_theorem : important_computation true false = false.
Proof.
auto.
Qed.
*)
