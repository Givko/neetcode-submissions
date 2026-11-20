class Solution:
    def isPalindrome(self, s: str) -> bool:
        i = 0
        s = s.lower()
        i_end = len(s)-1
        while i < i_end:
            c = s[i]
            c_end = s[i_end]
            if not c.isalnum():
                i += 1
                continue
            
            if not c_end.isalnum():
                i_end-=1
                continue

            if c != c_end:
                return False
            i+=1
            i_end-=1

        return True