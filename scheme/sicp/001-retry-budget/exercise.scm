;; 001: 斷線重連，總共要等多久？
;; n 與 delay-ms 都是非負整數；結果單位是毫秒。
;; 'todo 是刻意留下的佔位值，不是解答。

(define (retry-wait-recursive n delay-ms)
  ;; TODO: 產生 linear recursive process。
  'todo)

(define (retry-wait-iterative n delay-ms)
  ;; TODO: 自訂輔助程序，以狀態參數產生 linear iterative process。
  'todo)

;; 我的觀察：
;; 1. (3, 100) 的兩種過程：
;; TODO
;; 2. 呼叫自己與 iterative process 的關係：
;; TODO
;; 3. 步數與待處理操作：
;; TODO
