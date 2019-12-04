# Day 4

A nice easy problem, but if the numbers were larger, it could have been a very challenging problem. At first I did some hacky mess to finish, but then I realized I could just check if it was sorted to ensure it was nondecreasing. So I refined it so that it went through every number in the range, converting each to a list of digits. Then I filtered that according to the first part and saved it as a vec, then filtered that for the second part.

But then I realized I could just iterate over all combinations and use the range as a secondary constraint. The final version runs 6 times faster than the version that looked at every number.
