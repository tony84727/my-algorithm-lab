# 001：斷線重連，總共要等多久？

**約 15–25 分鐘 · SICP 1.2.1 · 不需要 list、parser 或 `eval`**

遊戲連線失敗後，客戶端會等一下再重試。第一次等 `delay-ms` 毫秒，
之後每次的等待時間都是前一次的兩倍。你想在畫面上顯示：
「如果接下來重試 `n` 次，總共會花多少時間在等待？」

例如，重試 3 次、第一次等 100 毫秒，就會依序等 100、200、400 毫秒，總共 700 毫秒。
這只是計算模型；不真的等待或發出網路請求，也先不加入上限、抖動或請求耗時。

## 你的任務

打開 [exercise.scm](exercise.scm)，完成兩個程序：

```scheme
(retry-wait-recursive n delay-ms)
(retry-wait-iterative n delay-ms)
```

兩者都回傳總等待毫秒數，並遵守相同規則：

- `n` 與 `delay-ms` 都是非負整數，不必處理非法輸入
- `n` 是接下來的重試次數，每次重試前都有一次等待；原本已失敗的請求不計入
- `n = 0` 時不需要等待，回傳 0
- 第一版產生 linear recursive process，留下等較小問題回傳後才完成的加法
- 第二版產生 linear iterative process，用固定數量的狀態參數前進，遞迴呼叫位於 tail position
- 用 `define`、`if`、`=`、`+`、`-`、`*` 與自訂輔助程序即可；這次先不用 `expt`、等比級數公式、list、`set!` 或迴圈語法

不要把第二版寫成呼叫第一版；試著真正換一種記住「目前進度」的方法。

## 先手算，再執行

| `n` | `delay-ms` | 等待時間 | 回傳值 |
| --- | --- | --- | --- |
| 0 | 100 | 不等待 | 0 |
| 1 | 100 | 100 | 100 |
| 3 | 100 | 100 + 200 + 400 | 700 |
| 4 | 250 | 250 + 500 + 1000 + 2000 | 3750 |
| 5 | 0 | 每次都等 0 | 0 |

從 repository 根目錄執行：

```sh
cd scheme/sicp/001-retry-budget
mit-scheme --quiet --batch-mode --load test.scm
```

起始碼故意回傳 `'todo`，第一次應看到 **16 個 FAIL** 並以非零狀態結束。
兩個程序完成後應看到 **16 checks, 0 failed**，exit status 為 0。
測試只驗證回傳值，無法證明你的程序產生哪一種 process。
測試工具使用的 `set!` 不屬於你需要實作的部分。

想單獨試一個輸入，可以在同一個資料夾執行 `mit-scheme`，然後：

```scheme
(load "exercise.scm")
(retry-wait-recursive 3 100)
(retry-wait-iterative 3 100)
```

若使用 Guile，同一份檔案也可用 `guile --no-auto-compile -s test.scm` 執行。
測試工具已在 MIT/GNU Scheme 12.1 與 GNU Guile 3.0.10 驗證。

## 最後兩分鐘

把簡短觀察寫在 `exercise.scm` 末尾的註解裡：

1. 用 `(3, 100)` 追蹤兩版的過程：第一版留下哪些還沒做的加法？第二版每一步帶著什麼狀態？
2. 第二版也會呼叫自己，為什麼仍然是 iterative process？
3. 以處理一次等待為一步，兩版各走幾步？當 `n` 增加時，哪一版的待處理操作會累積？

這裡比較的是步數、待處理操作與狀態參數個數；精確整數本身仍可能隨數值變大而占用更多記憶體。

## 卡住再打開

<details>
<summary>提示 1：先拿走一次等待</summary>

做完第一次等待後，還剩幾次重試？剩下那個較小問題的「第一次等待時間」是多少？
先確定 `n = 0` 的情況，再想第一版要把什麼留到回傳後處理。

</details>

<details>
<summary>提示 2：把進度帶在身上</summary>

第二版的輔助程序可以記住三件事：剩餘次數、下一次等待多久、已累積多久。
先在紙上列出 `(3, 100)` 的初始狀態與下一個狀態，確認每個值的意思都沒改變。

</details>

<details>
<summary>提示 3：檢查回來之後還有沒有工作</summary>

如果輔助程序回傳後，外層還要再做 `+`，那個加法仍是待處理工作。
能不能在下一次呼叫之前，把這次等待計入狀態？

</details>

回書裡對照：SICP §1.2.1「Linear Recursion and Iteration」，尤其是 recursive procedure
與 recursive process 的區別。可參考 [MIT 提供的原書](https://web.mit.edu/6.001/6.037/sicp.pdf)。
