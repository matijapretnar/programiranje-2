let a () =
  print_endline "Začenjam A";
  for i = 1 to 5 do
    print_endline ("A " ^ string_of_int i)
  done

let b () =
  for i = 1 to 10 do
    print_endline ("B " ^ string_of_int i)
  done

(* ----------------------- *)

type stanje_a =
  | ZacetekA
  | PredIzpisomA of int
  | KonecA

let a_asinh = function
  | ZacetekA ->
      print_endline "Začenjam A";
      PredIzpisomA 1
  | PredIzpisomA i ->
      print_endline ("A " ^ string_of_int i);
      if i < 5 then PredIzpisomA (i + 1) else KonecA
  | KonecA -> KonecA

let a_kot_smo_ga_navajeni () =
  let rec ponavljaj stanje =
    match a_asinh stanje with
    | KonecA -> ()
    | stanje' -> ponavljaj stanje'
  in
  ponavljaj ZacetekA

type stanje_b =
  | ZacetekB
  | PredIzpisomB of int
  | KonecB

let b_asinh = function
  | ZacetekB -> PredIzpisomB 1
  | PredIzpisomB i ->
    print_endline ("B " ^ string_of_int i);
    if i < 10 then PredIzpisomB (i + 1) else KonecB
  | KonecB -> KonecB

type cakajoc =
  | CakaA of stanje_a
  | CakaB of stanje_b

let rec poganjaj =
  function
  | [] -> ()
  | (CakaA KonecA | CakaB KonecB) :: ostali ->
      poganjaj ostali
  | CakaA stanje :: ostali ->
      let stanje' = a_asinh stanje in
      poganjaj (ostali @ [CakaA stanje'])
  | CakaB stanje :: ostali ->
      let stanje' = b_asinh stanje in
      poganjaj (ostali @ [CakaB stanje'])

let _ = poganjaj [CakaA ZacetekA; CakaB ZacetekB]
