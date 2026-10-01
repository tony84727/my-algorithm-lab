# Scheme 小實驗

用小問題搭配 SICP，先動手，再回書裡看程式產生的 process。
每題只留題目、起始碼與測試，解答由自己補上。

## 環境

使用 [MIT/GNU Scheme](https://www.gnu.org/software/mit-scheme/)。先確認：

```sh
mit-scheme --version
```

已用 MIT/GNU Scheme 12.1 與 GNU Guile 3.0.10 驗證測試工具，不需要額外 Scheme 套件。
現有 Rust、C、Python
工作流程沒有執行這裡的測試，請依各題 README 的指令執行。

## 題目

- [001：斷線重連，總共要等多久？](sicp/001-retry-budget/) — SICP 1.2.1，recursive / iterative process，約 15–25 分鐘

## 完成條件

合併出題 PR 只代表收下題目，不代表已完成。

完成時補上實作與題目要求的簡短觀察，執行測試，再提交解題 commit。
可以在 PR 留下該 commit 與測試結果，或直接說這題已完成。
