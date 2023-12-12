from enum import Enum
from typing import Self
import re

class CubeColor(Enum):
    RED = 1
    GREEN = 2
    BLUE = 3

class CubeStack:
    def __init__(self, color: CubeColor, count: int):
        self.color = color
        self.count = count

    def from_str(string: str) -> Self:
        string = string.strip()
        segments: list[str] = string.split(" ")
        return CubeStack(CubeColor[segments[1].upper()], int(segments[0]))

class GameRound:
    def __init__(self, id: int, cube_stacks: list[CubeStack]) -> None:
        self.id = id
        self.cube_stacks = cube_stacks

    def from_str(string: str) -> Self:
        string = string.strip()
        colon_split: list[str] = string.split(':')
        id: str = colon_split[0]
        id = id.split(' ')[1]
        cube_stacks: list[CubeStack] = [CubeStack.from_str(stack) for stack in re.split(',|;', colon_split[1])]
        return GameRound(int(id), cube_stacks)

    def get_max(self, color: CubeColor) -> int:
        max: int = 0
        for stack in self.cube_stacks:
            if stack.color != color: continue
            if stack.count > max: max = stack.count
        return max


INPUT_FILE = "input.txt"

with open(INPUT_FILE, 'r') as file:
    input: str = file.read()
    output: int = 0
    rounds: list[GameRound] = [GameRound.from_str(line) for line in input.split('\n') if len(line) > 1]
    for round in rounds:
        max_red: int = round.get_max(CubeColor.RED)
        max_green: int = round.get_max(CubeColor.GREEN)
        max_blue: int = round.get_max(CubeColor.BLUE)
        output += max_red * max_green * max_blue
    print('Output: ' + str(output))