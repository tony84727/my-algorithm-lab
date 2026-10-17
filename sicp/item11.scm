;; Item11: Fermat checks over a supplied range of bases
;; Contract: n is an exact integer >= 2.
;; Contract: first-base is an exact integer from 1 through n.
;; Contract: last-base is an exact integer from 0 through n - 1.
;; Check every integer base in the inclusive range.
;; Return #t for an empty range.
;; Return #f immediately when a base fails the Fermat equality.
;; A #t result does not prove that n is prime.

;; Provided utility: modular exponentiation.
;; Do not edit this utility for today's exercise.
(define (expmod base exponent modulus)
  (define (step current-base remaining product)
    (cond ((= remaining 0) product)
          ((even? remaining)
           (step (remainder (* current-base current-base) modulus)
                 (/ remaining 2)
                 product))
          (else
           (step current-base
                 (- remaining 1)
                 (remainder (* product current-base) modulus)))))
  (step (remainder base modulus) exponent 1))

(define (fermat-range-pass? n first-base last-base)
  ;; TODO: Decide when there are no bases left to check.
  ;; TODO: Check the Fermat equality for the current base.
  ;; TODO: Stop on failure; otherwise continue with the next base.
  (if (> first-base last-base)
      #f
      (if (= (expmod first-base n n) first-base)
	  (fermat-range-pass? n (+ first-base) last-base)
	  #f)))

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
  (display "Fermat range tests")
  (newline)
  (check-case "smallest prime" (fermat-range-pass? 2 1 1) #t)
  (check-case "prime subrange" (fermat-range-pass? 7 2 5) #t)
  (check-case "all bases for a prime" (fermat-range-pass? 13 1 12) #t)
  (check-case "base one can pass" (fermat-range-pass? 9 1 1) #t)
  (check-case "single failing base" (fermat-range-pass? 9 2 2) #f)
  (check-case "pass then fail" (fermat-range-pass? 9 1 2) #f)
  (check-case "one misleading base" (fermat-range-pass? 15 4 4) #t)
  (check-case "include the last base" (fermat-range-pass? 15 4 7) #f)
  (check-case "empty range at n" (fermat-range-pass? 2 2 1) #t)
  (check-case "Carmichael subrange" (fermat-range-pass? 561 2 10) #t)
  (check-case "base shares a factor" (fermat-range-pass? 561 3 3) #t)
  (check-case "all bases still pass" (fermat-range-pass? 561 1 560) #t))

;; Manual check:
;; Trace (fermat-range-pass? 9 1 2).
;; Record each tested base and its modular-power remainder.
;; Check that a failing base ends the search immediately.
;; Explain why a #t result for 561 does not establish primality.
