# Completely Randomized Design

完全隨機設計的重點在於：讓實驗單位（E.U.）完全隨機的分配 trt 以避免實驗單位間的差異性。這樣的設計方式可以讓我們更容易的去估計 trt 的效應。e.g. 實驗的唯一因素有 3 個 level，每個 level 進行 2 次實驗，共進行 6 次實驗。這 6 次實驗的順序應該是完全隨機，而不是固定的例如 1,2,3,1,2,3。

假設有 $k$ 個 trt，每個 trt 分別收集 $n_j$ 個觀察值，實驗數據可以建模為：$j=1,\cdots,k$; $i=1,\cdots,n_j$

$$
Y_{ij} = \mu + \tau_j + \varepsilon_{ij},\quad \varepsilon_{ij}\overset{\text{iid}}{\sim}N(0,\sigma^2_{\varepsilon})
$$

- $Y_{ij}$: $j$-th trt 的 $i$-th 觀測值。
- $\mu$: 整個實驗的共同效應，是未知的定值。
- $\tau_j$: $j$-th trt 的效應，是未知的定值。
- $\varepsilon_{ij}$: 隨機誤差項

Note: $\mu_j=\mu+\tau_j$ 代表 $j$-th trt 的平均效應，是未知的定值。

:::warning
以上下標存在符號濫用嫌疑。
:::

## 2 factor CRD Factorial experment

假設有 $a$ 個水平的 factor A 和 $b$ 個水平的 factor B，每個 trt 收集 $n_{ij}$ 個觀測值。可以將原始模型擴展為：

$$
Y_{ijk}= \mu_{ij}+\varepsilon_{ijk},\quad \varepsilon_{ijk}\overset{\text{iid}}{\sim}N(0,\sigma^2_{\varepsilon}),\quad \begin{align*}
i&=1,\cdots,a\\
j&=1,\cdots,b\\
k&=1,\cdots,n_{ij}
\end{align*}
$$

$$
\implies \mu_{ij}=E(Y_{ijk})=E(Y|A=i,B=j)
$$

## Factorial vs. OFAT

Factorial 實驗是對每個 treatment 都進行至少一次實驗，而 OFAT (One Factor At a Time) 則通過與選擇的 baseline trt 進行比較，來估計其他 trt 的效應，具體有以下步驟：
1. 選擇一個 baseline trt
2. 固定 B 因素，遍歷 A 因素的所有水平 => 得到最佳的 A 水平，記作 $i_A$
3. 固定 A 因素，遍歷 B 因素的所有水平 => 得到最佳的 B 水平，記作 $i_B$
4. 得到最佳 trt 組合 $(i_A,i_B)$

OFAT 並不會查看所有 trt 的組合，因此可能會錯過一些重要的交互作用。$(i_A, j_B)$ 也可能並沒有觀測值。

EX: Y 為某集成電路的電流量，有以下因素，假設希望數值越大越好：
- A: 海拔（$0^k, 3^k$）
- B: 溫度（25, 55）

以 OFAT 方式實驗收集到以下數據：

![alt text](img/crd/ofat.png)

假設 baseline trt 為 $(0^k, 25)$，則：
- 固定 B 因素，$\bar{Y}:207.5\to 235\Rightarrow 27.5$
- 固定 A 因素，$\bar{Y}:207.5\to 182.5\Rightarrow -25$

得到結論，trt $(0^k, 55)$ 是最佳的。

以 Factorial 方式實驗收集到以下數據：

![alt text](img/crd/factorial.png)

比較所有觀測值，得到結論，trt $(0^k, 55)$ 是最佳的。

兩種方法得到了相同的結論，但 Factorial 只需要 4 次實驗，OFAT 需要 6 次實驗。

如果我們將 B 因素設為 x 軸，觀察值設為 y 軸，並將 A 因素相同 level 的點連接起來，可以得到以下圖形：

![alt text](img/crd/graphical_factorial.png)

可以從圖中看到 A 和 B 之間存在交互左右，因為不同 A 水平下會影響線條的斜率。而這是 OFAT 無法發現的。

## Graphical display of data

$$
Y_{ijk}=\mu+\tau_i+\beta_j+(\tau\beta)_{ij}+\varepsilon_{ijk}
$$

我們可以通過圖形化的方式來展示 $\mu_{ij}$ 的效應：x 軸為 B 因素，y 軸為 $\mu_ij$ 的效應，將 A 因素相同 level 的點連接起來。

如果 A 的 level 數 $a=2$，B 的 level 數 $b=2$，則有下列 3 中可能的圖像：

![alt text](img/crd/graph_of_mu.png)

1. 平行（Parallel）
   
   $\implies $ A 的效應與 B 的 level 無關，B 的效應與 A 的 level 無關。

   $\implies$ A 和 B 之間不存在交互作用。
2. 相交（Crossed）
   
   $\implies$ A 的效應隨 B 的 level 改變，B 的效應隨 A 的 level 改變。

   $\implies$ A 和 B 之間存在強烈的交互作用。

3. 介於 Parallel 和 Crossed 之間

   $\implies$ 可能存在或不存在交互左右，需要進一步檢驗。

Note：$\mu_{ij}$ 是未知真實值，但可以通過對 trt(i,j) 的觀察值進行平均來估計。i.e.

$$
\widehat{\mu_{ij}}=\frac{1}{n_{ij}}\sum_{k=1}^{n_{ij}}Y_{ijk}=\bar{Y}_{ij\cdot}
$$

## Interaction

Note: A 與 B 如果不存在交互作用 $\iff$
- $\mu_{ij}-\mu_{i'j}=\mu_{ij'}-\mu_{i'j'}\quad\forall i\neq i', j\neq j'$, i.e. A 的效應與 B 的 level 無關。
- $\mu_{ij}-\mu_{ij'}=\mu_{i'j}-\mu_{i'j'}\quad\forall i\neq i', j\neq j'$, i.e. B 的效應與 A 的 level 無關。

合併以上兩點得到：

$$
\begin{align*}
    &\mu_{ij}+\mu_{i'j'}-\mu_{ij'}-\mu_{i'j}=0 \quad\forall i\neq i', j\neq j'\\
    \implies&\frac{1}{a\cdot b}\sum_{i'=1}^a\sum_{j'=1}^b(\mu_{ij}+\mu_{i'j'}-\mu_{ij'}-\mu_{i'j})=0\quad\forall i=1,\cdots,a, j=1,\cdots,b\\
    \implies&\mu_{ij}+\bar{\mu}_{\cdot\cdot}-\bar{\mu}_{i\cdot}-\bar{\mu}_{\cdot j}=0\quad \forall i=1,\cdots,a, j=1,\cdots,b
\end{align*}
$$

$$
\text{with}\quad \bar{\mu}=\bar{\mu}_{\cdot\cdot}=\frac{1}{a\cdot b}\sum_{i=1}^a\sum_{j=1}^b\mu_{ij},\quad \bar{\mu}_i=\bar{\mu}_{i\cdot}=\frac{1}{b}\sum_{j=1}^b\mu_{ij},\quad \bar{\mu}_j=\bar{\mu}_{\cdot j}=\frac{1}{a}\sum_{i=1}^a\mu_{ij}
$$

- $\mu$: Grand mean (整個實驗的平均效應)
- $\mu_i$: E(Y|A=i) (A 的效應)
- $\mu_j$: E(Y|B=j) (B 的效應)

Remark: $H_0:\mu_{ij}+\bar{\mu}-\bar{\mu}_i-\bar{\mu}_j=0 \forall i,j$ 如果被拒絕，則代表 $H_0: $ “A，B 之間不存在交互作用”被拒絕。

:::tip[Definition]
$$
AB_{ij}=\mu_{ij}+\mu-\mu_i-\mu_j
$$

is called the interation of $A=i$ and $B=j$.
:::

Main effect: A 因素從 level $i\to i'$ 的效應。

$$
\begin{align*}
    \frac{1}{b}\sum_{j=1}^b\mu_{ij}-\frac{1}{b}\sum_{j=1}^b\mu_{i'j}&=\bar{\mu}_{i\cdot}-\bar{\mu}_{i'\cdot}=\mu_i-\mu_{i'}\\
    &=(\mu_i-\mu)-(mu_{i'}-\mu)\\
    &=A_i-A_{i'}
\end{align*}
$$

:::tip[Definition]
Main effect of A at level $i\triangleq \mu_i-\mu\quad i=1,\cdots,a$ donate $A_i$

Main effect of B at level $j\triangleq \mu_j-\mu\quad j=1,\cdots,b$ donate $B_j$
:::

模型也可以由以上定義進行解釋：

$$
\begin{align*}
    Y_{ijk}&=\mu_{ij}+\varepsilon_{(ij)k}\\
    &=\mu+(\mu_{ij}+\mu-\mu_i-\mu_j)+(\mu_i-\mu)+(\mu_j-\mu)+\varepsilon_{(ij)k}\\
    &=\mu+A_i+B_j+AB_{ij}+\varepsilon_{(ij)k}\\
\end{align*}
$$

## Fixed and Random effects

Effect 有兩種類型：
- Fixed effect：對特定的固定值感興趣。
- Random effect：對母體感興趣，而樣本只是其中的一部分。

對於不同類型效應的組合，可以形成不同的實驗模型，並且會對其加上不同的限制：
1. A, B both fixed effect $\iff$ *fixed model*
   
   $\implies$ Added： 

   $$
    \sum_{i=1}^aA_i=0,\quad \sum_{j=1}^bB_j=0\\ 
    \sum_{i=1}^aAB_{ij}=0,\quad\forall j,\quad \sum_{j=1}^bAB_{ij}=0,\quad\forall i
   $$

   這代表：A 有 $a-1$ 個 degree of freedom，B 有 $b-1$ 個 degree of freedom，AB 有 $(a-1)(b-1)$ 個 degree of freedom。
2. A, B both random effect $\iff$ *random model*
   
    $\implies$ Added：
    
    $$
     A_1,\cdots,A_a\overset{\text{iid}}{\sim}N(0,\sigma^2_A),\quad B_1,\cdots,B_b\overset{\text{iid}}{\sim}N(0,\sigma^2_B)\\
     AB_{ij}\overset{\text{iid}}{\sim}N(0,\sigma^2_{AB})\quad i=1,\cdots,a, j=1,\cdots,b
    $$

    且 $A_i\perp B_i\perp AB_{ij}\perp \varepsilon_{(ij)k}$

    Note: $\sigma^2_A=0\implies A_1=\cdots=A_a=0$, i.e. No A effect
    
3. One fixed, one random $\iff$ *mixed model*
   
   $\implies$ Added:

   $$
    \sum A_i=0,\quad B_1,\cdots,B_b\overset{\text{iid}}{\sim}N(0,\sigma^2_B), \quad AB_{ij}\overset{\text{iid}}{\sim}N(0,\sigma^2_{AB})\\
    \text{with } \sum_{i=1}^a A_iB_j=0\quad\forall j,\quad \sum_{j=1}^b A_iB_j\neq 0\quad \forall i\quad \text{ usually}
   $$