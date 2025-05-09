Lecture Notes: 17 Threads · Classes w/ Prof. Nat Tuck



[↓Skip to main content](#main-content)

[Classes w/ Prof. Nat Tuck](/)

* [Home](/)
* [cs2370](/classes/2025-01/cs2370/)
* [cs2470](/classes/2025-01/cs2470/)
* [cs4310](/classes/2025-01/cs4310/)
* [Inkfish](https://inkfish.homework.quest/)

Lecture Notes: 17 Threads
=========================

2025 Mar 03·Updated: 2025 Mar 04·4 mins

Virtual Memory and Fork: A Review [#](#virtual-memory-and-fork-a-review)
------------------------------------------------------------------------

* Draw the virtual memory diagram.
* Allocate some shared memory.
* Fork.
* Point out that shared memory is shared, and non-shared writable memory
  soon isn’t.

Introducing Threads [#](#introducing-threads)
---------------------------------------------

```
// create.c
#include <stdio.h>
#include <pthread.h>
#include <assert.h>

#define NN 10

void*
thread_main(void* thread_arg)
{
    int xx = *((int*)thread_arg);
    printf("thread %d: We're in a thread.\n", xx);
    *((int*)thread_arg) += xx;
    return thread_arg;
}

int
main(int _argc, char* _argv[])
{
    int nums[NN];
    int rv;
    pthread_t threads[NN];

    printf("main: Starting %d threads.\n", NN);

    for (int ii = 0; ii < NN; ++ii) {
        nums[ii] = ii;

        rv = pthread_create(&(threads[ii]), 0, thread_main, &(nums[ii]));
        assert(rv == 0);
    }

    printf("main: Started %d threads.\n", NN);

    for (int ii = 0; ii < NN; ++ii) {
        void* ret;
        rv = pthread_join(threads[ii], &ret);

        int yy = *((int*) ret);
        printf("main: Joined thread %d, rv = %d.\n", ii, yy);
    }

    printf("main: All threads joined.\n");

    return 0;
}

```

* show create.c
* Discuss how threads change the virtual memory story.

Threads vs. Processes [#](#threads-vs-processes)
------------------------------------------------

* We can spawn multiple processes with fork()
* We can execute multiple threads within a single process.

Key difference: With threads, all memory is shared by default.

* Advantage: Allocating shared memory post-spawn.
* Disadvantage: 100% data races

### History: [#](#history)

#### Early days [#](#early-days)

* Before multi-processor systems parallelism didn’t matter.
* Concurrency was still useful though:
  + Running multiple programs at once.
  + Having multiple logical tasks happening within one program.
* On Unix style systems, processes were commonly used for concurrency.
* On early Windows / Mac systems, concurrency within a program was represented
  by cooperative threading:
  + One thread could run at a time.
  + To let other threads run, explicitly call yield()
  + Some systems had an implicit yield when a thread blocked on I/O.
* By the 90’s, systems had some sort of pre-emptive threading. This still didn’t
  work in parallel, but it would automatically schedule work between threads
  without explicit yield() calls.

#### Multiprocessors [#](#multiprocessors)

* Multiprocessor servers became widely available in the mid 90’s.
* Windows and Solaris had decent parallel thread support.
* Linux didn’t get fully functional threads until like 2002, so fork() was
  heavily optimized instead.
* Result: Threads are much more efficient than processes on Windows.
* Threads under Linux evolved from fork(), so the performance difference
  is small.
* Multi-core desktop processors showed up around 2005, and suddenly
  parallelism became nessisary for performance.

Conditon Variables [#](#conditon-variables)
-------------------------------------------

* Stack
* Condvar stack

Other stuff [#](#other-stuff)
-----------------------------

These things are a usually bad idea compared to just using mutexes:

* show atomic-sum101; compare to mutex and parallel versions
* write it with pthread\_spin\_lock
* write our own spinlock with atomic\_compare\_exchange\_strong
  + need to google the docs; too new for a manpage
  + bad ideas include sched\_yield

```
// sequential stack

#include <pthread.h>
#include <stdio.h>
#include <assert.h>
#include <unistd.h>

int stack[5];
int stptr = 0;

void
stack_push(int xx)
{
    stack[stptr++] = xx;
}

int
stack_pop()
{
    return stack[stptr--];
}

int
main(int _ac, char* _av[])
{
    for (int ii = 0; ii < 5; ++ii) {
        stack_push(ii);
    }

    for (int ii = 0; ii < 5; ++ii) {
        int yy = stack_pop();
        printf("%d\n", yy);
    }

    return 0;
}

```

Parallel stack with cond vars:

```
#include <pthread.h>
#include <stdio.h>
#include <assert.h>
#include <unistd.h>
#include <stdlib.h>

#define STACK_SIZE 5
int stack[STACK_SIZE];
int stptr = 0;
pthread_mutex_t mutex;
pthread_cond_t  condv;

void
stack_push(int xx)
{
    pthread_mutex_lock(&mutex);
    while (stptr >= STACK_SIZE) {
        pthread_cond_wait(&condv, &mutex);
    }
    stack[++stptr] = xx;
    pthread_cond_broadcast(&condv);
    pthread_mutex_unlock(&mutex);
}

int
stack_pop()
{
    pthread_mutex_lock(&mutex);
    while (stptr <= 0) {
        pthread_cond_wait(&condv, &mutex);
    }
    int yy = stack[stptr--];
    pthread_cond_broadcast(&condv);
    pthread_mutex_unlock(&mutex);
    return yy;
}

void*
producer_thread(void* arg)
{
    int nn = *((int*) arg);
    free(arg);

    for (int ii = 0; ii < nn; ++ii) {
        stack_push(ii);
    }
}

int
main(int _ac, char* _av[])
{
    pthread_t threads[2];
    pthread_mutex_init(&mutex, 0);
    pthread_cond_init(&condv, 0);

    for (int ii = 0; ii < 2; ++ii) {
        int* nn = malloc(sizeof(int));
        *nn = 1000;
        int rv = pthread_create(&(threads[ii]), 0, producer_thread, nn);
        assert(rv == 0);
    }

    while (1) {
        int yy = stack_pop();
        printf("%d\n", yy);
        usleep(10000);
    }

    return 0;
}

```

![Nat Tuck](/img/author.jpg)

Author

Nat Tuck

---

[←→

Lecture Notes: 16 Data Races

2025 Mar 01](/classes/2025-01/cs4310/notes/17-sum101/)

[Lecture Notes: 20 Malloc Slides, pt2

2025 Mar 05
→←](/classes/2025-01/cs4310/notes/20-more-slides/)

[↑](#the-top "Scroll to top")

©
2025
Nat Tuck

Powered by [Hugo](https://gohugo.io/) & [Congo](https://github.com/jpanther/congo)