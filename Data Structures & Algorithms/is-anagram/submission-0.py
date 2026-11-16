from collections import Counter


class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        if len(s) != len(t):
            return False
        
        s_count = Counter(s)
        t_count = Counter(t)

        for letter, count in s_count.items():
            c: int | None = t_count.get(letter)
            if c is None or count != c:
                return False
        return True
        
        