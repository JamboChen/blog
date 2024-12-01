# Nested-Factor Design

e.g. 如果要判斷不同地區學生的能力，我們可以對每個地區抽 3 個學生做測試。這樣我們有兩個因素：4 個地區和 3 個學生。但每個地區選的學生不可能一樣，並且對於一個學生，他只能是他所在地區的一部分，而不可能轉移到其他地區。也就是說學生因素是**嵌套**在地區因素裡的。

:::info[Definition]
Factor $B$ is nested in factor A $\iff$ the levels of B are similar but not identical at each level of A.
:::

2 factors with B is nested in A:

$$
\begin{gather*}

    Y_[ijk]=\mu+A_i+B_{j(i)}+\varepsilon_{(ij)k}\\
    i=1,\cdots,a,\quad j=1,\cdots,b_i,\quad k=1,\cdots,n\quad\varepsilon_{(ij)k}\overset{\text{iid}}{\sim}N(0,\sigma^2)
\end{gather*}
$$

在這個實驗中不會存在 A 和 B 的交互作用，因為 A 的不同等級下，B 的等級是不一樣的。因此不會有 B 的某些效應在不同的 A 等級下的效果。

ANOVA with EMS:

|                       | df        | i(F/R) | j(F/R) | k(R) | A:F, B:F                        | A:F, B:R                                    | A:R, B:R                                        |
| --------------------- | --------- | ------ | ------ | ---- | ------------------------------- | ------------------------------------------- | ----------------------------------------------- |
| $A_i$                 | $a-1$     | $0/n$  | $b$    | $n$  | $\sigma_\varepsilon^2+nb\phi_A$ | $\sigma_\varepsilon^2+n\sigma_B^2+nb\phi_A$ | $\sigma_\varepsilon^2+n\sigma_B^2+nb\sigma^2_A$ |
| $B_{j}(i)$            | $a(b-1)$  | $1$    | $0/1$  | $n$  | $\sigma_\varepsilon^2+n\phi_B$  | $\sigma_\varepsilon^2+n\sigma_B^2$          | $\sigma_\varepsilon^2+n\sigma_B^2$              |
| $\varepsilon_{(ij)k}$ | $ab(n-1)$ | $1$    | $1$    | $1$  | $\sigma_\varepsilon^2$          | $\sigma_\varepsilon^2$                      | $\sigma_\varepsilon^2$                          |
| total                 | $abn-1$   |        |        |      |                                 |                                             |                                                 |

**Remark**: B 嵌套在 A 中時，通常因素 A 是我們關心的，因此 A 通常是固定的，而 B 通常是隨機的。

## ANOVA

|       | df      | SS          | SS  | MS  | F   | P   |
| ----- | ------- | ----------- | --- | --- | --- | --- |
| A(F)  | a-1     | $SS_A$      |     |     |     |     |
| B(F)  | a(b-1)  | $SS_{B(A)}$ |     |     |     |     |
| Error | ab(n-1) | $SS_E$      |     |     |     |     |
| Total | abn-1   | $SS_T$      |     |     |     |     |

$$
SS_{B(A)}=\sum_{i=1}^a(SS_B|A=i)
$$

$SS_A,SS_T$ 的計算與之前的一樣。

**EX**: 為了選出最好的供應商，要求 A,B,C 三家供應商提供 4 批次的產品，每批次 3 個樣本。我們要測試供應商和批次是否有影響。

| Suppliers | A   | A   | A   | A   | B   | B   | B   | B   | C   | C   | C   | C   |
| --------- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Batch     | 1   | 2   | 3   | 4   | 1   | 2   | 3   | 4   | 1   | 2   | 3   | 4   |
| Data 1    | 1   | -2  | -2  | 1   | 1   | 0   | -1  | 0   | 2   | -2  | 1   | 3   |
| Data 2    | -1  | -3  | 0   | 4   | -2  | 4   | 0   | 3   | 4   | 0   | -1  | 2   |
| Data 3    | 0   | -4  | 1   | 0   | -3  | 2   | -2  | 2   | 0   | 2   | 2   | 1   |
| Total     | 0   | -9  | -1  | 5   | -4  | 6   | -3  | 5   | 6   | 0   | 2   | 6   |

| Group | A   | B   | C   | Total |
| ----- | --- | --- | --- | ----- |
| Total | -5  | 4   | 14  | 13    |

$$
\begin{align*}
    SS_{Total}&=148.31\\
    SS_{Suppliers}&=\frac{(-5)^2+4^2+14^2}{12}-\frac{13^2}{36}=15.06\\
    SS_{Batch(Suppliers)}&=33.544_{(A)}+27.333_{(B)}+9_{(C)}=69.92\\
\end{align*}
$$

$\implies$ ANOVA

|           | df  | SS     | MS   | F    | P    |
| --------- | --- | ------ | ---- | ---- | ---- |
| Suppliers | 2   | 15.06  | 7.53 | 0.97 | 0.42 |
| Batch     | 9   | 69.92  | 7.77 | 2.94 | 0.02 |
| A         | 3   | 33.583 |      |      |      |
| B         | 3   | 27.333 |      |      |      |
| C         | 3   | 9      |      |      |      |
| Error     | 24  | 63.33  | 2.64 |      |      |

- $H_0:$ No Supplier effect $\implies$ Not reject
- $H_0:$ No Batch effect $\implies$ Reject

i.e. 在某個供應商下，不同批次的產品有顯著差異。$\implies$ A 供應商下的批次有顯著差異。