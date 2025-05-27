import sys


def is_solvable(result: str, equation: list[str]) -> bool:
    print(len(equation))
    if len(equation) == 1:
        print('testing')
        return int(result) == int(equation[0])
    print('not returning')
    *equation, last_part = equation
    # could the last part have been added to get the result?
    result, last_part = int(result), int(last_part)
    # assume last operator was +
    if is_solvable(result - last_part, equation):
        return True
    str_result = str(result)
    str_last_part = str(last_part)
    if str_result.endswith(str_last_part):
        str_result = str_result[0:len(str_result)-len(str_last_part)]
        print(str_result, 'from', str_last_part, 'of', result)
        if len(str_result) > 0 and is_solvable(str_result, equation):
            print('solvable')
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
