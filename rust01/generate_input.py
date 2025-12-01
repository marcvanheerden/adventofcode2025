#!/usr/bin/env python3
import random
import sys

def generate_input(num_lines: int, max_value: int = 1000) -> str:
    lines = []
    for _ in range(num_lines):
        direction = random.choice(['L', 'R'])
        value = random.randint(1, max_value)
        lines.append(f"{direction}{value}")
    return '\n'.join(lines)

if __name__ == "__main__":
    num_lines = int(sys.argv[1]) if len(sys.argv) > 1 else 100
    max_value = int(sys.argv[2]) if len(sys.argv) > 2 else 1000
    print(generate_input(num_lines, max_value))
