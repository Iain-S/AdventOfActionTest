"""Print a string to stdout."""
import sys

with open("input.txt") as f:
    lines = f.read().splitlines()

numbers = [l.split("   ") for l in lines]

list1 = sorted([int(n[0]) for n in numbers])
list2 = sorted([int(n[1]) for n in numbers])

if sys.argv[1] == "one":
    # PART 1
    differences = [abs(l2 - l1) for l1, l2 in zip(list1, list2)]
    print(sum(differences))

else:
    # PART 2
    similarity_score = 0
    for n in list1:
        similarity_score += n * list2.count(n)
    print(similarity_score)
