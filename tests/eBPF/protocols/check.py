import argparse


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='Parse eBPF data.')
    parser.add_argument('--protocol', type=str, help='protocol name')

    args = parser.parse_args()
    actual = f"{args.protocol}/result/actual.txt"
    expected = f"{args.protocol}/result/expected.txt"
    actual = open(actual, 'r', encoding="utf-8")
    expected = open(expected, 'r', encoding="utf-8")

    actual_lines = actual.read().split('\n')
    expected_lines = expected.read().split('\n')
    all = min(len(expected_lines), len(actual_lines)) - 1
    same = 0
    for i in range(all):
        if actual_lines[i] == expected_lines[i]:
            same += 1
        else:
            print(f"actual: {actual_lines[i]}")
            print(f"expected: {expected_lines[i]}")
            print("{}", i)
    print(f"acc: {same / all}")

    # Close the files after reading
    actual.close()
    expected.close()