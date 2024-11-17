# Randomized Complete Block Design (RCBD)

這個實驗方法的策略是通過將實驗單元分組，消除組（Block）間可能出現的變異，以此來增加實驗的準確性。

## One factor

**EX**: 有 4 種牌子的輪胎：A,B,C,D，$Y=$ 跑 20000 公裡後的磨損量，我們想知道哪個牌子的輪胎最好。

$\implies$ factor: 4 levels and is fixed

- Design 1: 4 台車，每台車裝 1 種牌子的輪胎。
  
  這是一個不好的設計，因為輪子品牌的效應與車的效應混在一起，具有強相關性。

- Design 2 (CRD): 16 個輪胎完全隨機的分配到 4 台車的 4 個位置上。
  
  $Y_{ij}=\mu+\tau_i+\varepsilon_{ij}$, 其中 $\tau_i$ 代表輪子的效應。收集到以下數據：

  | Car1       | Car2       | Car3       | Car4       |
  | ---------- | ---------- | ---------- | ---------- |
  | $C\mid 12$ | $A\mid 14$ | $C\mid 10$ | $A\mid 13$ |
  | $A\mid 17$ | $A\mid 13$ | $D\mid 11$ | $D\mid 9$  |
  | $D\mid 11$ | $B\mid 14$ | $B\mid 14$ | $B\mid 8$  |
  | $D\mid 14$ | $C\mid 12$ | $B\mid 13$ | $C\mid 9$  |

  ANOVA:

  | Source | SS    | df  | MS    | F    | p-value |
  | ------ | ----- | --- | ----- | ---- | ------- |
  | Brand  | 30.69 | 3   | 10.23 | 2.44 | 0.115   |
  | Error  | 50.25 | 12  | 4.19  |      |         |
  | Total  | 80.94 | 15  |       |      |         |

  i.e. 四個牌子的輪胎的平均磨損量沒有顯著差異。

  這個設計中同樣沒有控制車的效應。

- Design 3 (RCBD): 為了消除因為車帶來的潛在的變異
  
  | Car1       | Car2       | Car3       | Car4       |
  | ---------- | ---------- | ---------- | ---------- |
  | $C\mid 12$ | $A\mid 14$ | $C\mid 10$ | $A\mid 13$ |
  | $A\mid 17$ | $A\mid 13$ | $D\mid 11$ | $D\mid 9$  |
  | $D\mid 11$ | $B\mid 14$ | $B\mid 14$ | $B\mid 8$  |
  | $D\mid 14$ | $C\mid 12$ | $B\mid 13$ | $C\mid 9$  |
  
RCBD 由以下幾個部分組成：
1. 每組包含所有的 trt。
2. 在一個組中，trt 隨機分配到實驗單元上。

設 trt 的數量為 $k$

| Block 1  | Block 2  | $\cdots$ | Block b  |
| -------- | -------- | -------- | -------- |
| $\pi_1$  | $\pi_1$  | $\cdots$ | $\pi_1$  |
| $\pi_2$  | $\pi_2$  | $\cdots$ | $\pi_2$  |
| $\vdots$ | $\vdots$ | $\ddots$ | $\vdots$ |
| $\pi_a$  | $\pi_a$  | $\cdots$ | $\pi_a$  |

其中 $(\pi_1,\cdot,\pi_a)$ 是 $(1,\cdots,a)$ 的隨機排列。

當我們得到具體數據：

| Block 1   | Block 2   | $\cdots$ | Block b   |
| --------- | --------- | -------- | --------- |
| $Y_{11k}$ | $Y_{12k}$ | $\cdots$ | $Y_{14k}$ |
| $Y_{21k}$ | $Y_{22k}$ | $\cdots$ | $Y_{24k}$ |
| $\vdots$  | $\vdots$  | $\vdots$ | $\vdots$  |
| $Y_{a1k}$ | $Y_{a2k}$ | $\cdots$ | $Y_{a4k}$ |

並建模為：

$$
\begin{gather*}
   Y_{ijk}=\mu+\tau_i+\beta_j+\varepsilon_{(ij)k} \\
   i=1,\cdots,a,\quad j=1,\cdots,b,\quad k=1,\cdots,n\xlongequal{\text{usually}}1
\end{gather*}
$$

- $\tau_i$: trt 的效應
- $\beta_j$: block 的效應

我們通常會假設 trt 與 block 直接沒有交互作用。並且 block effect 通常假設為 random effect，以這個例子來說，這樣假設可以將沒有實驗的車種的效應也納入考慮。

$\implies$ 以上數據的 ANOVA(RCBD)：

| Source | df  | SS    | MS   | F   | p-value         |
| ------ | --- | ----- | ---- | --- | --------------- |
| Brand  | 3   | 30.69 | 10.2 | 7.8 | $P(F_{3,9}>)=0$ |
| Block  | 3   | 38.69 | 12.9 |     |                 |
| Error  | 9   | 11.56 | 1.3  |     |                 |
| Total  | 15  | 80.94 |      |     |                 |

$\implies H_0: $ No brand effect 可以在 5% 的顯著水準下被拒絕。 
