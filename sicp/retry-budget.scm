;;; Retry budget: recursive and iterative processes in MIT Scheme.
;;; n is a nonnegative integer. The first n waits are 1, 2, 4, 8, ... .
;;; Return the total waiting time, or 0 when n = 0.
;;; From this directory, start MIT Scheme and enter:
;;;   (load "retry-budget.scm")
;;;   (run-tests)
;;; Loading this file only defines procedures; it does not run the tests.

;; Recursive process: defer each addition until the recursive call returns.
(define (retry-budget-rec n)
  (cond ((equal? n 0) 0)
        (else (+ (expt 2 (- n 1))
                 (retry-budget-rec (- n 1))))))

;; Iterative process: carry the current index and accumulated total.
(define (retry-budget-iter-internal i n total)
  (if (= i n)
      total
      (retry-budget-iter-internal (+ i 1) n (+ total (expt 2 i)))))

(define (retry-budget-iter n)
  (if (= n 0)
      0
      (retry-budget-iter-internal 0 n 0)))

;; Compare the actual and expected values and print each result.
(define (check-case name procedure n expected)
  (let ((actual (procedure n)))
    (display (if (equal? actual expected) "PASS " "FAIL "))
    (write (list name n))
    (display " expected: ")
    (write expected)
    (display " actual: ")
    (write actual)
    (newline)))

(define (run-tests)
  (for-each
   (lambda (entry)
     (for-each
      (lambda (test)
        (check-case (car entry) (cadr entry) (car test) (cadr test)))
      '((0 0) (1 1) (3 7) (4 15))))
   (list (list 'retry-budget-rec retry-budget-rec)
         (list 'retry-budget-iter retry-budget-iter))))
