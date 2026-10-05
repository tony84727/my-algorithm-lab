;;; count-ways.scm
;;; Count ordered ways to climb n stairs using steps of 1 or 2.
;;; n is a non-negative integer. There is one way to climb 0 stairs.
;;; Run: (load "count-ways.scm") then (run-tests)

(define (count-ways n)
  ;; TODO: Add base cases and combine the smaller problems.
  'TODO)

(define (check-case n expected)
  (let ((actual (count-ways n)))
    (display (if (equal? actual expected) "PASS " "FAIL "))
    (write (list 'count-ways n))
    (display " expected: ")
    (write expected)
    (display " actual: ")
    (write actual)
    (newline)))

(define (run-tests)
  (for-each
   (lambda (test)
     (check-case (car test) (cadr test)))
   '((0 1) (1 1) (2 2) (4 5) (6 13))))

;;; After the tests pass, trace (count-ways 4).
;;; Identify one input that gets computed more than once.
