;; Day 06: Fast multiplication with a recursive process
;; Input: a and b are non-negative integers.
;; Output: the product of a and b.
;; Use addition, doubling, and halving.
;; Do not use multiplication, expt, or mutation.

(define (double n)
  (+ n n))

(define (halve n)
  ;; Precondition: n is a positive even integer.
  (quotient n 2))

(define (fast-multiply a b)
  ;; TODO: Handle zero groups.
  ;; TODO: For positive even b, solve half the groups
  ;;       and double the returned result.
  ;; TODO: For odd b, solve one fewer group
  ;;       and add the remaining group after the call returns.
  ;; Make only one recursive call in each non-base branch.
  (cond ((= b 0) 0)
	((even? b) (fast-multiply (double a) (halve b)))
	(else (+ a (fast-multiply a (- b 1))))
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
  (check-case "zero groups" (fast-multiply 13 0) 0)
  (check-case "zero items per group" (fast-multiply 0 9) 0)
  (check-case "one group" (fast-multiply 9 1) 9)
  (check-case "even groups" (fast-multiply 7 6) 42)
  (check-case "odd groups" (fast-multiply 7 5) 35)
  (check-case "power of two groups" (fast-multiply 13 16) 208)
  (check-case "swapped inputs" (fast-multiply 6 7) 42))

;; Manual check:
;; Trace (fast-multiply 3 5).
;; Mark operations that wait for a recursive result.
