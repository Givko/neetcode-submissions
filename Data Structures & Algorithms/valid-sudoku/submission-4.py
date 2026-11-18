class Solution:
    def getSquareNum(self, row: int,col: int):
        if row >=0 and row <= 2:
            if col >= 0 and col <= 2:
                return 0
            if col >= 3 and col <= 5:
                return 3
            if col >=6 and col<=8:
                return 6
        elif row >=3 and row <= 5:
            if col >= 0 and col <= 2:
                return 1
            if col >= 3 and col <= 5:
                return 4
            if col >=6 and col<=8:
                return 7
        elif row >= 6 and row <=8:
            if col >= 0 and col <= 2:
                return 2
            if col >= 3 and col <= 5:
                return 5
            if col >=6 and col<=8:
                return 8

        raise IndexError

            
    def isValidSudoku(self, board: list[list[str]]) -> bool:
        rows: dict[int, list[str]] = {i: [] for i in range(9)} 
        cols: dict[int, list[str]] = {i: [] for i in range(9)} 
        squares:  dict[int, list[str]] = {i: [] for i in range(9)} 
        for i, row in enumerate(board):
            for j, item in enumerate(row):
                if item == ".":
                    continue
                if item in rows[i]:
                    return False
                if item in cols[j]:
                    return False
                square = self.getSquareNum(i,j)
                if item in squares[square]:
                    return False
                rows[i].append(item)
                cols[j].append(item)
                squares[square].append(item)

        return True
