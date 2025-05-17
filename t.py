#!/usr/bin/env python3
import subprocess
import sys
import tomllib
import pathlib
import dataclasses
import difflib


@dataclasses.dataclass
class TestCase:
    input: str
    output: str

    def check(self, argc: list[str]) -> bool:
        input = self.input.strip() + "\n"
        done = subprocess.run(args=argc, stdout=subprocess.PIPE, input=input, text=True)
        actual = done.stdout.strip()
        expected = self.output.strip()
        if actual == expected:
            print("test case OK", file=sys.stderr)
            return True
        else:
            print("test case failed", file=sys.stderr)
            split = lambda text: list(map(lambda s : s + '\n', text.split('\n')))
            diffs = difflib.unified_diff(
                split(expected), split(actual), fromfile="expected.out", tofile="actual.out"
            )
            sys.stdout.writelines(diffs)
            return False


def load_testcases(path: pathlib.Path) -> list[TestCase]:
    testcases_str = path.read_text()
    testcases_dict = tomllib.loads(testcases_str)
    testcases: list[TestCase] = []
    for tc_dict in testcases_dict["testcases"]:
        testcases.append(TestCase(**tc_dict))
    return testcases


def main(argv: list[str]):
    print(argv, file=sys.stderr)
    testcases = load_testcases(pathlib.Path("./testcases.toml"))
    ok = True
    for testcase in testcases:
        if not testcase.check(argv[1:]):
            ok = False
    if ok:
        sys.exit(0)
    else:
        sys.exit(1)


if __name__ == "__main__":
    main(sys.argv)
