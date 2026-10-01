;; 從本資料夾執行：mit-scheme --quiet --batch-mode --load test.scm
;; 這個檔案是測試工具，不需要為了解題修改它。
(load "exercise.scm")

(define checks 0)
(define failures 0)

(define (check name expected actual)
  (set! checks (+ checks 1))
  (if (equal? expected actual)
      (begin (display "PASS ") (display name) (newline))
      (begin
        (set! failures (+ failures 1))
        (display "FAIL ") (display name)
        (display ": expected ") (write expected)
        (display ", got ") (write actual)
        (newline))))

(define (check-both name n delay-ms expected)
  (check (string-append "recursive: " name)
         expected (retry-wait-recursive n delay-ms))
  (check (string-append "iterative: " name)
         expected (retry-wait-iterative n delay-ms)))

(check-both "no retries" 0 100 0)
(check-both "zero retries and delay" 0 0 0)
(check-both "one retry" 1 100 100)
(check-both "two retries" 2 75 225)
(check-both "example: three retries" 3 100 700)
(check-both "example: four retries" 4 250 3750)
(check-both "zero delay" 5 0 0)
(check-both "ten retries" 10 7 7161)

(newline)
(display checks) (display " checks, ")
(display failures) (display " failed") (newline)
(exit (if (= failures 0) 0 1))
