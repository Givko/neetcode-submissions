class Solution:
    def groupAnagrams(self, strs: list[str]) -> list[list[str]]:
      anagram_groups = []
      str_group_map = {}
      for word in strs:
        if len(anagram_groups) == 0:
          sorted_word = ''.join(sorted(word))
          str_group_map[sorted_word] = len(anagram_groups)
          anagram_groups.append([word]) 
          continue

        sorted_word = ''.join(sorted(word))
        group_index = str_group_map.get(sorted_word)
        if group_index is not None:
           anagram_groups[group_index].append(word)
        else:
           str_group_map[sorted_word] = len(anagram_groups)
           anagram_groups.append([word]) 
        
      return anagram_groups
