;; Day 03: Count stair-climbing ways with an iterative process
;; Input: n is a non-negative integer.
;; Each move climbs exactly 1 or 2 stairs.
;; Different move orders count as different ways.
;; There is one way to climb zero stairs: make no moves.
;; Goal: Use a fixed number of state variables and a tail call.
;; Do not call the tree-recursive count-ways procedure.
;; No lists, arrays, memoization, or mutation are needed.

(define (count-ways-iter n)
  ;; TODO: Choose initial values for all three helper arguments.
  ;; Replace the placeholder with a call to count-ways-step.
  (count-ways-step n 1 1))

(define (count-ways-step remaining ways-here ways-next)
  ;; remaining: the number of forward steps still needed.
  ;; ways-here: the number of ways to reach the current stair.
  ;; ways-next: the number of ways to reach the next stair.
  ;; TODO: Choose the stopping condition and returned state value.
  ;; TODO: Advance all three state values in one tail call.
  ;; Compute new arguments from the current argument values.
  (if (<= remaining 0)
    ways-here
    (count-ways-step (- remaining 1) ways-next (+ ways-here ways-next))
  ))

(define (check-case label actual expected)
  (display label)
  (if (equal? actual expected)
      (display " PASS")
      (begin
        (display " FAIL: expected ")
        (write expected)
        (display ", got ")
        (write actual)))
  (newline))

(define (run-tests)
  (check-case "zero stairs" (count-ways-iter 0) 1)
  (check-case "one stair" (count-ways-iter 1) 1)
  (check-case "two stairs" (count-ways-iter 2) 2)
  (check-case "four stairs" (count-ways-iter 4) 5)
  (check-case "six stairs" (count-ways-iter 6) 13)
  (check-case "ten stairs" (count-ways-iter 10) 89))

;; Manual work:
;; 1. Trace (remaining, ways-here, ways-next) for n=4.
;; 2. Check the meaning of every state value after each step.
;; 3. Explain why no addition is left pending after the tail call.
