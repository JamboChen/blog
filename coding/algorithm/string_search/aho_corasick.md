# Aho–Corasick 算法

[![Aho–Corasick_algorithm](https://upload.wikimedia.org/wikipedia/commons/thumb/9/90/A_diagram_of_the_Aho-Corasick_string_search_algorithm.svg/220px-A_diagram_of_the_Aho-Corasick_string_search_algorithm.svg.png)](https://en.wikipedia.org/wiki/Aho%E2%80%93Corasick_algorithm)


Aho–Corasick 算法可以在給定文本中定位靜態字典中所有字串的位置。在搜索時它會同時檢查字典中所有的字串，並且在 O(n + m + z) 的時間複雜度內找到所有匹配的位置，其中 n 是文本的長度，m 是字典中所有字串的總長度，z 是所有匹配的數量。

該算法的重點在於，它在前綴樹（Trie）的基礎上對每個節點添加了失敗指針（failure link），當在某個分支上匹配失敗時，可以迅速轉移到下一個潛在的匹配位置，而不用回溯到根節點重新匹配。

## 算法思路

字典中一個字串的某些子字串，可能是其他字串的前綴。例如字典中有 'abcd' 和 'bc' 兩個字串，而匹配文本為 'abce'，當指針在 'abcd' 的 'c' 將要匹配 'd' 時失敗，此時可以直接跳轉到 'bc' 的 'c' 重新開始匹配，而不用回溯到 'a' 重新開始匹配。

