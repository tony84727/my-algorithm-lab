(** https://ocaml.org/problems *)
let rec last = function
        | [] -> None
        | [h] -> Some h
        | h :: t -> last t

let rec last_two = function
        | [] -> None
        | [a;b] -> Some (a, b)
        | h :: t -> last_two t

let encode s =
  let merge result c =
    match result with
    | (current, current_count) :: t when current = c ->
        (current, current_count + 1) :: t
    | x -> (c, 1) :: x
  in
  let rec calc s =
    match s () with Seq.Nil -> [] | Seq.Cons (h, t) -> merge (calc t) h
  in
  calc (String.to_seq s)

let rec nth l n =
  match (l, n) with
  | [], _ -> raise (Failure "nth")
  | h :: _, 0 -> h
  | _ :: t, x -> nth t (x - 1)

let rec length = function [] -> 0 | _ :: t -> 1 + length t

let rev l =
  let rec aux acc = function [] -> acc | h :: t -> aux (h :: acc) t in
  aux [] l

let is_palindrome l = l = List.rev l

type 'a node = One of 'a | Many of 'a node list

let rec flatten = function
  | [] -> []
  | One a :: t -> a :: flatten t
  | Many nested :: t -> flatten nested @ flatten t

let rec compress = function
  | a :: (b :: _ as t) -> if a = b then compress t else a :: compress t
  | smaller -> smaller

let rec pack =
  let merge a = function
    | (b :: _ as h) :: t when a == b -> (a :: h) :: t
    | x -> [ a ] :: x
  in
  function [] -> [] | h :: t -> merge h (pack t)

type 'a rle = One of 'a | Many of int * 'a

let rec mencode =
  let merge a = function
    | One b :: t when a = b -> Many (2, a) :: t
    | Many (count, b) :: t when a = b -> Many (count + 1, b) :: t
    | x -> One a :: x
  in
  function [] -> [] | h :: t -> merge h (mencode t)

let rec duplicate = function [] -> [] | h :: t -> h :: h :: duplicate t

let replicate l x =
  let rec repeat item = function 0 -> [] | x -> item :: repeat item (x - 1) in
  let rec r count = function
    | [] -> []
    | h :: t -> repeat h count @ r count t
  in
  r x l

let drop l c =
  let rec drop it = function
    | [] -> []
    | _ :: t when it + 1 = c -> drop 0 t
    | h :: t -> h :: drop (it + 1) t
  in
  drop 0 l
