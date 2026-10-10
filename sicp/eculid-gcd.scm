;; Item 08: Euclid's algorithm and greatest common divisors
;; Input: a and b are exact non-negative integers.
;; At least one input must be positive; (0, 0) is out of scope.
;; Output: the greatest common divisor as a positive integer.
;; Both input orders must work. Do not call the built-in gcd.
;; Use remainder, comparisons, conditionals, and tail calls.
;; Do not use mutation or search through possible divisors.

(define (euclid-gcd a b)
  ;; TODO: Choose a stopping condition before computing a remainder.
  ;; TODO: Return the correct result at that stopping condition.
  ;; TODO: Choose the next pair using Euclid's observation.
  ;; Keep the greatest common divisor unchanged at each step.
  ;; Make the recursive call the final operation in its branch.
  (if (or (= a 0) (= b 0))
      (+ a b)
      (let ((a (max a b)) (b (min a b))) (euclid-gcd b (remainder a b)))))

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
  (check-case "common factor" (euclid-gcd 48 18) 6)
  (check-case "smaller input first" (euclid-gcd 18 48) 6)
  (check-case "square tile size" (euclid-gcd 60 24) 12)
  (check-case "coprime inputs" (euclid-gcd 35 64) 1)
  (check-case "equal inputs" (euclid-gcd 42 42) 42)
  (check-case "exact division" (euclid-gcd 54 9) 9)
  (check-case "second input is zero" (euclid-gcd 17 0) 17)
  (check-case "first input is zero" (euclid-gcd 0 17) 17)
  (check-case "book example" (euclid-gcd 206 40) 2)
  (check-case "several reductions" (euclid-gcd 1071 462) 21))

;; Manual check:
;; Trace each numeric (a, b) state for (euclid-gcd 206 40).
;; Count the remainder operations actually evaluated.
;; Check that every state has the same greatest common divisor.
;; Explain why no work remains after each recursive call returns.
