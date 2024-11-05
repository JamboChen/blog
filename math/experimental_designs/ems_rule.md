---
sidebar_position: 3
---

# EMS rule

## EMS rule for balanced designs

for model

$$
Y_{ijk} = \mu + A_i + B_j + AB_{ij} + \epsilon_{{ij}k}
$$

$i=1,\cdots,a,\quad j=1,\cdots,b,\quad k=1,\cdots,n,\quad\varepsilon_{(ij)k}\overset{\text{iid}}{\sim}N(0,\sigma^2)$

根據以下過程製作一張表格：
1. 準備一張表格，row 為所有 factor 、交互作用和隨機項目，column 為所有下標及其對應的 factor 是隨機或固定。
   |                       | i,F | j,R | k,R |
   | --------------------- | --- | --- | --- |
   | $A_i$                 |     |     |     |
   | $B_j$                 |     |     |     |
   | $AB_{ij}$             |     |     |     |
   | $\varepsilon_{(ij)k}$ |     |     |     |


2. 對於每個 column，如果所對應下標並不在 effect 中，則填充 level 數量。
   |                       | i,F | j,R | k,R |
   | --------------------- | --- | --- | --- |
   | $A_i$                 |     | b   | n   |
   | $B_j$                 | a   |     | n   |
   | $AB_{ij}$             |     |     | n   |
   | $\varepsilon_{(ij)k}$ |     |     |     |


3. 將每個 row 裡所有在括號中的下標位置填充 1 。
   |                       | i,F | j,R | k,R |
   | --------------------- | --- | --- | --- |
   | $A_i$                 |     | b   | n   |
   | $B_j$                 | a   |     | n   |
   | $AB_{ij}$             |     |     | n   |
   | $\varepsilon_{(ij)k}$ | 1   | 1   |     |

4. 對於每個 column，如果下標所對應的 factor 是隨機的，則填充 1，如果是固定的，填充 0。
   |                       | i,F | j,R | k,R |
   | --------------------- | --- | --- | --- |
   | $A_i$                 | 0   | b   | n   |
   | $B_j$                 | a   | 1   | n   |
   | $AB_{ij}$             | 0   | 1   | n   |
   | $\varepsilon_{(ij)k}$ | 1   | 1   | 1   |

接下來根據以下規則計算 EMS：
1. 忽略該 trt 下標中，括號之外的所有 column （e.g. $A_i$ 忽略 $i$ col，$\varepsilon_{(ij)k}$ 忽略 $k$ col）。
2. 找到所有包含該 trt 下標的所有 row，將其對應的 column 值相乘。
3. 篩選出的 row 中，如果包含隨機效應，則其對應的方差為 $\sigma_\tau$，如果都是固定效應，則其對應的方差為 $\phi_\sigma$。
4. 將 2. 和 3. 的結果相乘，並將每個 row 的結果相加，即為 EMS。

|                       | i,F | j,R | k,R | EMS                                            |
| --------------------- | --- | --- | --- | ---------------------------------------------- |
| $A_i$                 | 0   | b   | n   | $\sigma_\varepsilon^2+n\sigma^2_{AB}+bn\phi_A$ |
| $B_j$                 | a   | 1   | n   | $\sigma_\varepsilon^2+an\sigma_B$              |
| $AB_{ij}$             | 0   | 1   | n   | $\sigma_\varepsilon^2+n\sigma_{AB}$            |
| $\varepsilon_{(ij)k}$ | 1   | 1   | 1   | $\sigma_\varepsilon^2$                         |

## ANOVA table with EMS

| Source | SS           | DF                 | MS                 | EMS                                            | F-value        | $H_0$              |
| ------ | ------------ | ------------------ | ------------------ | ---------------------------------------------- | -------------- | ------------------ |
| A      | $a-1$        | $SS_A$             | $MS_A$             | $\sigma_\varepsilon^2+n\sigma^2_{AB}+bn\phi_A$ | $MS_A/MS_{AB}$ | $A$ has no effect  |
| B      | $b-1$        | $SS_B$             | $MS_B$             | $\sigma_\varepsilon^2+an\sigma_B$              | $MS_B/MS_E$    | $B$ has no effect  |
| AB     | $(a-1)(b-1)$ | $SS_{AB}$          | $MS_{AB}$          | $\sigma_\varepsilon^2+n\sigma_{AB}$            | $MS_{AB}/MS_E$ | $AB$ has no effect |
| Error  | $ab(n-1)$    | $SS_{\varepsilon}$ | $MS_{\varepsilon}$ | $\sigma_\varepsilon^2$                         |                |                    |
| Total  | $N-1$        |                    |                    |                                                |                |                    |

F-value 的分子需要只比分母多出需要檢定效應的方差項。

**Remark**: if $n=1$, i.e. one observation each treatment

$$
\implies SS_E=\sum_i\sum_j\sum_k^1(Y_{ijk}-\bar{Y}_{ij.})^2=0 \quad\text{with }df=ab(n-1)=0
$$

i.e. $MS_E$ is not defined.

---

**EX**: 

$$
Y_{ijkl} = \mu + A_i + B_j + C_k + AB_{ij} + AC_{ik} + BC_{jk} + ABC_{ijk} + \varepsilon_{(ijk)l}
$$

|                        | i,R | j,R | k,R | l,R | EMS                                                                                  | F-value            |
| ---------------------- | --- | --- | --- | --- | ------------------------------------------------------------------------------------ | ------------------ |
| $A_i$                  | 1   | b   | c   | n   | $\sigma_\varepsilon^2+n\sigma^2_{ABC}+cn\sigma^2_{AB}+bn\sigma^2_{AC}+bcn\sigma^2_A$ |                    |
| $B_j$                  | a   | 1   | c   | n   | $\sigma_\varepsilon^2+n\sigma^2_{ABC}+an\sigma^2_{BC}+cn\sigma^2_{AB}+acn\sigma^2_B$ |                    |
| $C_k$                  | a   | b   | 1   | n   | $\sigma_\varepsilon^2+n\sigma^2_{ABC}+an\sigma^2_{BC}+bn\sigma^2_{AC}+abn\sigma^2_C$ |                    |
| $AB_{ij}$              | 1   | 1   | c   | n   | $\sigma_\varepsilon^2+n\sigma^2_{ABC}+cn\sigma^2_{AB}$                               | $MS_{AB}/MS_{ABC}$ |
| $AC_{ik}$              | 1   | b   | 1   | n   | $\sigma_\varepsilon^2+n\sigma^2_{ABC}+bn\sigma^2_{AC}$                               | $MS_{AC}/MS_{ABC}$ |
| $BC_{jk}$              | a   | 1   | 1   | n   | $\sigma_\varepsilon^2+n\sigma^2_{ABC}+an\sigma^2_{BC}$                               | $MS_{BC}/MS_{ABC}$ |
| $ABC_{ijk}$            | 1   | 1   | 1   | n   | $\sigma_\varepsilon^2+n\sigma^2_{ABC}$                                               | $MS_{ABC}/MS_E$    |
| $\varepsilon_{(ijk)l}$ | 1   | 1   | 1   | 1   | $\sigma_\varepsilon^2$                                                               |                    |

$\implies$ $H_0:\sigma^2_A=0/\sigma^2_B=0/\sigma^2_C=0$ 並沒有檢定方法。因為沒有可以作為基礎的 EMS。

**Remark**: 如果 EMS rule 無法給出 effect 的檢定方法。可以用以下兩種方式：
1. 假設一些 effect/intraction 為 0 。
2. 用漸進方法得到 F-test。

**Remakr**: $\varepsilon_{(ij)k}$ 與 $\varepsilon_{ijk}$ 兩種符號的意義是不同的。前者表示 $k$ 在 $ij$ 效應下得到，意味著獲得數據的 trt 是隨機出現的，後者則代表數據是輪流獲得的。

**Remark**: 計算 SS 

$$
\begin{align*}
    (n-1)S^2&=\sum^n(X_i-\bar{X})^2\\
    &=\sum^nX_i^2-n\bar{X}^2\\
    &=\sum^nX_i^2-n\left(\frac{1}{n}\sum^nX_i\right)^2\\
    &=\sum^nX_i^2-\frac{1}{n}\left(\sum^nX_i\right)^2
\end{align*}
$$

$$
\begin{align*}
    \implies SS_E&=\sum_i^k\sum_j^n(Y_{ij}-\bar{Y}_{i\cdot})^2\\
    &=\sum_i^k\sum_j^nY_{ij}^2-\frac{1}{n}\sum_i^kY_{i\cdot}^2\\
    SS_{trt}&=\sum_i^k\sum_j^n(Y_{i\cdot}-\bar{Y}_{\cdot\cdot})^2\\
    &=\sum_i^k\frac{Y_{i\cdot}^2}{n}-\frac{Y_{\cdot\cdot}^2}{N}
\end{align*}
$$