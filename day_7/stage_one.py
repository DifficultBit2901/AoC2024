import sys


def is_solvable(result: str, equation: list[str]) -> bool:
    if len(equation) == 1:
        return int(result) == int(equation[0])
    *equation, last_part = equation
    result, last_part = int(result), int(last_part)
    # assume last operator was +
    if is_solvable(result - last_part, equation):
        return True
    # can last operator be * with ints?
    if (result // last_part) * last_part != result:
        return False
    # assume last operator was *
    if is_solvable(result // last_part, equation):
        return True
    return False


def main():
    file_name = sys.argv[1]
    with open(file_name) as f:
        count = 0
        for line in f:
            line = line.strip()
            result, equation = line.split(': ')
            if is_solvable(result, equation.split(' ')):
                count += int(result)

        print("Result:", count)


if __name__ == '__main__':
    main()
