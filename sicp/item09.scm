;; Item09: Smallest divisors and a trial-division prime test
;; smallest-divisor accepts an exact integer n >= 2.
;; trial-prime? accepts any exact integer and returns #t or #f.
;; Use arithmetic, remainder, comparisons, conditionals, and calls.
;; Do not use sqrt, mutation, lists, or a built-in prime test.
;; Try candidates 2, 3, 4, 5, ... and stop at the square boundary.

(define (smallest-divisor n)
  (find-divisor n 2))

(define (square x) (* x x))
(define (divides? divisor n) (= (remainder n divisor) 0))

(define (find-divisor n candidate)
  ;; All integers from 2 up to candidate - 1 have already failed.
  ;; TODO: Decide when there are no candidates left to test.
  ;; TODO: Decide when the current candidate is the answer.
  ;; TODO: Otherwise move to the next candidate in tail position.
  (cond ((> candidate (square candidate)) n)
	((divides? candidate n) candidate)
	(else (find-divisor n (+ candidate 1)))))

(define (trial-prime? n)
  ;; TODO: Handle integers below 2 before calling smallest-divisor.
  ;; TODO: Use smallest-divisor to produce a boolean result.
  (cond ((< n 2) #f)
	(else (= (smallest-divisor n) n))))

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
  (display "Smallest divisor tests")
  (newline)
  (check-case "smallest prime" (smallest-divisor 2) 2)
  (check-case "odd prime" (smallest-divisor 3) 3)
  (check-case "even square" (smallest-divisor 4) 2)
  (check-case "odd square" (smallest-divisor 9) 3)
  (check-case "square boundary" (smallest-divisor 49) 7)
  (check-case "exercise 1.21 A" (smallest-divisor 199) 199)
  (check-case "exercise 1.21 B" (smallest-divisor 1999) 1999)
  (check-case "exercise 1.21 C" (smallest-divisor 19999) 7)
  (display "Prime predicate tests")
  (newline)
  (check-case "negative integer" (trial-prime? -7) #f)
  (check-case "zero" (trial-prime? 0) #f)
  (check-case "one" (trial-prime? 1) #f)
  (check-case "two" (trial-prime? 2) #t)
  (check-case "three" (trial-prime? 3) #t)
  (check-case "nine" (trial-prime? 9) #f)
  (check-case "forty-nine" (trial-prime? 49) #f)
  (check-case "ninety-seven" (trial-prime? 97) #t)
  (check-case "larger prime" (trial-prime? 1999) #t)
  (check-case "larger composite" (trial-prime? 19999) #f))

;; Manual check:
;; Trace candidate and candidate * candidate for n = 49 and n = 47.
;; Mark the candidates that actually cause a remainder operation.
;; Explain why candidate * candidate = n must still be tested.
;; Identify the tail call and describe what is known at each step.
