;; Day 04: Fast exponentiation with an iterative process
;; Input: base is a positive integer.
;;        exponent is a non-negative integer.
;; Output: base raised to exponent.
;; Use successive squaring and tail calls.
;; Do not use the built-in expt procedure or mutation.

(define (fast-power base exponent)
  ;; TODO: Choose initial values for factor, remaining, and acc.
  ;; Replace this placeholder with a call to power-step.
  (power-step base exponent 1))

(define (square x) (* x x))
(define (power-step factor remaining acc)
  ;; Invariant: acc * factor^remaining equals the target answer.
  ;; TODO: Return the answer when remaining is zero.
  ;; TODO: For a positive even remaining, use squaring and halving.
  ;; TODO: For an odd remaining, move one factor into acc.
  ;; Keep the invariant true in every branch.
  ;; Make each recursive call a tail call.
  (cond ((= remaining 0) acc)
	((even? remaining) (* acc (square (power-step factor (/ remaining 2) 1))))
	(else (* acc (power-step factor (- remaining 1) factor)))
  )
)


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
  (check-case "zero exponent" (fast-power 7 0) 1)
  (check-case "one exponent" (fast-power 5 1) 5)
  (check-case "even exponent" (fast-power 2 6) 64)
  (check-case "odd exponent" (fast-power 3 5) 243)
  (check-case "larger exponent" (fast-power 2 10) 1024)
  (check-case "unit base" (fast-power 1 50) 1))

;; Manual work:
;; 1. Trace (factor, remaining, acc) for (fast-power 2 5).
;; 2. Check acc * factor^remaining at every state.
;; 3. Explain why nothing remains to be multiplied after a tail call.
