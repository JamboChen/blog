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

**Remark**: 
1. *fixed* effect 通常比 *random* effect 更重要。
2. F 統計量的分母的自由度越高，則檢定的 power 越高。


## When to use RCBD

在處理 paired data 時，我們將每對數據視為一個 block，來進行 RCBD。

AONVA with $n=1\implies N=ab$

- RCBD: $Y_{ij}=\mu+\tau_i+\beta_j\varepsilon_{ij}$
- CRD: $Y_{ij}=\mu+\tau_i+\varepsilon_{ij}$
  
|       | df(RCBD)   | df(RCBD) |
| ----- | ---------- | -------- |
| Trt   | a-1        | a-1      |
| Block | b-1        |          |
| Error | (a-1)(b-1) | a(b-1)   |
| Total | ab-1       | ab-1     |

$$
\begin{align*}
  \implies& SS_{E,CRD}=SS_{E,RCBD}+SS_{Block} \\
  \implies& \hat{\sigma}^2_{\varepsilon,CRD}=\frac{SS_{E,CRD}}{a(b-1)}=\frac{SS_{E,RCBD}+SS_{Block}}{a(b-1)} \\
\end{align*}
$$

$\implies$ 比較 RCBD 和 CRD 產生的 $\hat{\sigma}^2_{\varepsilon}$ 的差異。如果有很大差異，則代表 RCBD 的效果很好。如果沒有差異，代表我們沒有必要使用 RCBD。

e.g. 

|       | df  | SS    | MS   |
| ----- | --- | ----- | ---- |
| Brand | 3   | 30.69 | 10.2 |
| Block | 3   | 38.69 | 12.9 |
| Error | 9   | 11.56 | 1.3  |
| Total | 15  | 80.94 |      |


$$
\implies \hat{\sigma}^2_{\varepsilon,CRD}=\frac{38.69+11.56}{3+9}=4.1875\implies \frac{\hat{\sigma}^2_{\varepsilon,CRD}}{\hat{\sigma}^2_{\varepsilon,RCBD}}=\frac{4.1875}{1.3}=3.22
$$

RCBD 的效果比 CRD 好了 3.22 倍。也就是說，如果想要 CRD 有和 RCBD 一樣的效果，需要 3.22 倍的樣本數。

:::info[Definition]
*Nuisance* factor: 可能有影響但不感興趣的 factor。
:::

對於不同類型的 nuisance factor，我們可以使用不同的方法來處理：
1. 未知且不可控：使用隨機化來平衡其影響。
2. 已知但不可控：ANCOVA (Analysis of Covariance)。
3. 已知且可控：Blocking。

## Latin Square Design

在輪子的例子中，不僅車的不同可能會造成影響，輪子安裝位置的不同也可能有影響。如果兩個因素都考慮進去，那麼我們就會有兩個方向的 block，這就是 Latin Square Design。

:::info[Definition]
**$p\times p$ Latin Square Design**

1. 必須是正方形，其中 $p$ 是 trt 的數量。
2. 每行每列中，每個 trt 只能出現一次 （類似數獨）。
:::

e.g. Latin square 4x4

|     | 1   | 2   | 3   | 4   |
| --- | --- | --- | --- | --- |
| 1   | A   | B   | C   | D   |
| 2   | B   | C   | D   | A   |
| 3   | C   | D   | A   | B   |
| 4   | D   | A   | B   | C   |

不同邊長的 Latin square 的組合數是固定的。因此我們可以計算出所有可能的組合，然後隨機選擇一個，將對應的 trt 分配到實驗單元上。

等價的，我們也可以計算出最簡單的組合，然後將實驗單元隨機分配到每個格子上。

**One factor Latin Square Design**

$$
\begin{gather*}
  Y_{ijk}=\mu+\tau_i+\beta_j+\gamma_k+\varepsilon_{ijk}\\
  i=1,\cdots,p,\quad j=1,\cdots,p,\quad k=k(i,j)\in\{1,\cdots,p\}
\end{gather*}
$$
- $\tau$: 感興趣的效應
- $\beta$: 一個方向的 block 的效應
- $\gamma$: 另一個方向的 block 的效應

|     | total   | $\tau$ | $\beta$ | $\gamma$ | error        |
| --- | ------- | ------ | ------- | -------- | ------------ |
| df  | $p^2-1$ | $p-1$  | $p-1$   | $p-1$    | $(p-2)(p-1)$ |

繼續使用輪胎的例子，我們將輪子位置和車的效應都納入考慮：

| pos\car | 1         | 2         | 3         | 4         |     |
| ------- | --------- | --------- | --------- | --------- | --- |
| 1       | $C\mid12$ | $D\mid11$ | $A\mid13$ | $B\mid8$  | 44  |
| 2       | $B\mid14$ | $C\mid12$ | $D\mid11$ | $A\mid13$ | 50  |
| 3       | $A\mid17$ | $B\mid14$ | $C\mid10$ | $D\mid9$  | 50  |
| 4       | $D\mid13$ | $A\mid14$ | $B\mid13$ | $C\mid9$  | 49  |
|         | 56        | 51        | 47        | 39        | 193 |

$$
\implies SS_{pos}=\frac{44^2+50^2+50^2+49^2}{4}-\frac{193^2}{16}=6.19
$$

ANOVA (Latin Square):

|       | df  | SS    | MS      | F     | p-value |
| ----- | --- | ----- | ------- | ----- | ------- |
| Brand | 3   | 30.69 | 10.2292 | 11.42 | 0.007   |
| Car   | 3   | 38.69 | 12.8958 |       |         |
| Pos   | 3   | 6.19  | 2.062   |       |         |
| Error | 6   | 5.37  | 0.8958  |       |         |
| Total | 15  | 80.94 |         |       |         |

$\implies$ Brands 之間有顯著差異。

用 SNK test 來選擇最好的品牌：

1. $MS_E=0.8958$ with $df=6$ $\implies S_{\bar{Y}_{\cdot}}=\sqrt{\frac{MS_E}{n}}=\sqrt{\frac{0.8958}{4}}=0.047$
2.  
    | $p$             | 2    | 3    | 4   |
    | --------------- | ---- | ---- | --- |
    | $q_{0.05}(p,6)$ | 3.46 | 4.34 | 4.9 |
    | $SNK_{0.05}(p)$ | 1.63 | 2.04 | 2.3 |
3. 
   |             | C     | D     | B     | A     |
   | ----------- | ----- | ----- | ----- | ----- |
   | Sample mean | 10.75 | 11.00 | 12.25 | 14.25 |

   | diff&critical | D                | B                | A           |
   | ------------- | ---------------- | ---------------- | ----------- |
   | C             | $0.25\not >1.63$ | $1.5\not >2.4$   | $3.5>2.3 $  |
   | D             |                  | $1.25\not >1.63$ | $2.25>2.04$ |
   | B             |                  |                  | $2>1.63$    |

   $\implies$ A 显著大于其他品牌，而 B,C,D 之间没有显著差异。

