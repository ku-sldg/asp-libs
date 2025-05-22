(* ImportantTheoremTest.v *)

Set Warnings "-notation-overridden,-parsing".

Require Import String.

Parameter MISSING: Type.


Module Check.

Ltac check_type A B :=
    match type of A with
    | context[MISSING] => idtac "Missing:" A
    | ?T => first [unify T B; idtac "Type: ok" | idtac "Type: wrong - should be (" B ")"]
    end.

End Check.


Import Check.
Require Import ImportantTheorem.

Goal True.

idtac "#> test_important_theorem".
check_type @important_theorem ((important_computation true true = true)).
idtac "Assumptions:".
Abort.
Print Assumptions important_theorem.