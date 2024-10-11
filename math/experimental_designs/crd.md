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

兩種方法得到了相同的結論，但 Factorial 只需要 4 次實驗，OFAT 需要 6 次實驗。如果我們將 Factorial 的數據進行可視化：

![alt text](img/crd/graphical_factorial.png)

能看到 A 和 B 直接有強烈的交互作用，而 OFAT 並不能發現這一點。

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


## Graphical display of data