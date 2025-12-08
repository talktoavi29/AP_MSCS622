let to_int_list argv =
  Array.to_list argv
  |> List.tl
  |> List.map int_of_string

let mean lst =
  let sum = List.fold_left ( + ) 0 lst in
  (float_of_int sum) /. (float_of_int (List.length lst))

let median lst =
  let sorted = List.sort compare lst in
  let n = List.length sorted in
  if n mod 2 = 1 then
    float_of_int (List.nth sorted (n / 2))
  else
    let a = List.nth sorted ((n / 2) - 1) in
    let b = List.nth sorted (n / 2) in
    (float_of_int (a + b)) /. 2.0

let mode lst =
  let sorted = List.sort compare lst in
  let rec count_runs acc current count = function
    | [] -> (current, count) :: acc
    | x :: xs ->
        if x = current then
          count_runs acc current (count + 1) xs
        else
          count_runs ((current, count) :: acc) x 1 xs
  in
  match sorted with
  | [] -> ([], 0)
  | x :: xs ->
      let runs = count_runs [] x 1 xs |> List.rev in
      let max_freq =
        List.fold_left (fun m (_, c) -> max m c) 1 runs
      in
      if max_freq = 1 then ([], 0)
      else
        let modes =
          runs
          |> List.filter (fun (_, c) -> c = max_freq)
          |> List.map fst
        in
        (modes, max_freq)

let () =
  if Array.length Sys.argv < 2 then begin
    Printf.printf "Usage: %s <int> <int> ...\n" Sys.argv.(0);
    exit 0
  end;

  let lst = to_int_list Sys.argv in
  let n = List.length lst in
  let m = mean lst in
  let med = median lst in
  let (modes, freq) = mode lst in

  Printf.printf "Count: %d\n" n;
  Printf.printf "Mean: %.3f\n" m;
  Printf.printf "Median: %.3f\n" med;

  match modes with
  | [] -> Printf.printf "Mode: none\n"
  | _ ->
      let mode_str = String.concat ", " (List.map string_of_int modes) in
      Printf.printf "Mode(s): %s (frequency=%d)\n" mode_str freq
