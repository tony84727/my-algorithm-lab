;; Item 07: Fast multiplication with an iterative process
;; Input: a and b are non-negative integers.
;; Output: the product of a and b.
;; Use addition, doubling, halving, and tail calls.
;; Do not use multiplication, expt, or mutation.

(define (double n)
  (+ n n))

(define (halve n)
  ;; Precondition: n is a positive even integer.
  (quotient n 2))

(define (fast-multiply a b)
  ;; TODO: Choose the initial addend, remaining, and acc.
  ;; Replace this placeholder with a call to multiply-step.
  (multiply-step a b 0))

(define (multiply-step addend remaining acc)
  ;; All three inputs are non-negative integers.
  ;; Contract: return acc + addend * remaining.
  ;; TODO: Return the answer when remaining is zero.
  ;; TODO: Regroup a positive even number of groups.
  ;; TODO: Move one odd group into the accumulated total.
  ;; Preserve the contract when choosing the next state.
  ;; Make each recursive call a tail call.
  (cond ((= remaining 0) acc)
	((even? remaining) (multiply-step (double addend) (halve remaining) acc))
	(else (multiply-step addend (- remaining 1) (+ addend acc)))))

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
  (check-case "zero groups" (fast-multiply 13 0) 0)
  (check-case "zero items per group" (fast-multiply 0 9) 0)
  (check-case "one group" (fast-multiply 9 1) 9)
  (check-case "even groups" (fast-multiply 7 6) 42)
  (check-case "odd groups" (fast-multiply 7 5) 35)
  (check-case "power of two groups" (fast-multiply 13 16) 208)
  (check-case "swapped inputs" (fast-multiply 6 7) 42))

(define (run-state-tests)
  (check-case "finished state" (multiply-step 8 0 11) 11)
  (check-case "even state with accumulator"
              (multiply-step 4 6 8) 32)
  (check-case "odd state with accumulator"
              (multiply-step 3 5 2) 17))

;; Manual check:
;; Trace (addend, remaining, acc) for (fast-multiply 3 5).
;; Check that acc + addend * remaining stays equal to 15.
;; Mark the final operation in each non-base branch.
