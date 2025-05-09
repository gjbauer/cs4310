Lecture Notes: 04 Recipe for Assembly Code · Classes w/ Prof. Nat Tuck



[↓Skip to main content](#main-content)

[Classes w/ Prof. Nat Tuck](/)

* [Home](/)
* [cs2370](/classes/2025-01/cs2370/)
* [cs2470](/classes/2025-01/cs2470/)
* [cs4310](/classes/2025-01/cs4310/)
* [Inkfish](https://inkfish.homework.quest/)

Lecture Notes: 04 Recipe for Assembly Code
==========================================

Discuss the Homework [#](#discuss-the-homework)
-----------------------------------------------

* Pull it up on bottlenose
  + Write task in C
  + Then rewrite it in ASM
  + No compiler output
* Pull up solution directory
* Show calc running.

First: HW Questions [#](#first-hw-questions)
--------------------------------------------

* Homework questions?

Today: Assembly Recipie Examples [#](#today-assembly-recipie-examples)
----------------------------------------------------------------------

### Example 1: Collatz Conjecture [#](#example-1-collatz-conjecture)

* Start with an integer > 1.
* If even, divide by two.
* If odd, take 3\*x+1.
* Iterate repeatedly

The conjecture:

* All integers > 1 eventually get you to one.

Our program will take an input on the command line and
print the sequence to one and the number of iterations.

```
long
iterate(long x)
{
  if (x % 2 == 0) {
    return x/2;
  }
  else {
    return x*3 + 1;
  }
}

int
main(int argc, char* argv[])
{
  long x = atol(argv[1]);
  long i = 0; 
  
  while (x > 1) {
    printf("%ld\n", x);
    x = iterate(x);
    i++;
  }
  
  printf("i = %ld\n"):
  return 0;
}

```

Now, translate to ASM with the recipe.

Note patterns for “if” and “while” statements.

### Example 2: Fives [#](#example-2-fives)

<./fives.S>

* Key point here: Dealing with individual characters in strings.

### Bonus Examples: [#](#bonus-examples)

* [Sum Array C](./sum_array.c) => [Sum Array ASM](./sum_array.S)
* [Linked List ASM](./linked_list.S)

*There's no articles to list here yet.*

[↑](#the-top "Scroll to top")

©
2025
Nat Tuck

Powered by [Hugo](https://gohugo.io/) & [Congo](https://github.com/jpanther/congo)