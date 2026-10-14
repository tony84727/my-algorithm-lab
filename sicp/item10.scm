;; Item10: Modular exponentiation
;; Contract: base and exponent are exact integers >= 0.
;; Contract: modulus is an exact integer >= 2.
;; Return the remainder of base^exponent divided by modulus.
;; For this exercise, exponent = 0 returns 1, including base = 0.
;; Use recursive successive squaring and reduce after each product.
;; Do not use expt, fast-expt, mutation, or random numbers.
;; In the even branch, evaluate the smaller expmod call only once.

(define (square x)
  (* x x))

(define (expmod base exponent modulus)
  ;; TODO: Handle the zero, positive even, and odd exponent cases.
  ;; TODO: Use a smaller exponent and combine the returned remainder.
  ;; TODO: Reduce each combined product before returning it.
  (cond ((= exponent 0) 1)
	((= 0 base) 0)
	((even? exponent) (expmod (remainder (square base) modulus) (/ exponent 2) modulus))
	(else (remainder (* (expmod base (- exponent 1) modulus) base) modulus))
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
  (display "Modular exponentiation tests")
  (newline)
  (check-case "zero exponent" (expmod 2 0 17) 1)
  (check-case "zero to zero convention" (expmod 0 0 7) 1)
  (check-case "zero base" (expmod 0 5 7) 0)
  (check-case "base above modulus" (expmod 19 1 7) 5)
  (check-case "even exponent" (expmod 2 4 5) 1)
  (check-case "odd exponent" (expmod 3 5 7) 5)
  (check-case "target example" (expmod 2 10 17) 4)
  (check-case "next odd exponent" (expmod 2 11 17) 8)
  (check-case "mixed exponent bits" (expmod 4 13 497) 445)
  (check-case "base equals modulus" (expmod 7 2 7) 0)
  (check-case "small modulus" (expmod 12 8 5) 1)
  (check-case "large exponent" (expmod 2 1000 13) 3))

;; Manual check:
;; Trace (expmod 2 10 17).
;; Record each exponent on the way down and each remainder on return.
;; Verify that every returned value is between 0 and 16.
;; Check that the even branch does not repeat the same recursive call.
;; Identify the work that remains after each recursive call returns.
