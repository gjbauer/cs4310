Lecture Notes: 18 Condvar · Classes w/ Prof. Nat Tuck



[↓Skip to main content](#main-content)

[Classes w/ Prof. Nat Tuck](/)

* [Home](/)
* [cs2370](/classes/2025-01/cs2370/)
* [cs2470](/classes/2025-01/cs2470/)
* [cs4310](/classes/2025-01/cs4310/)
* [Inkfish](https://inkfish.homework.quest/)

Lecture Notes: 18 Condvar
=========================

2025 Mar 05·Updated: 2025 Mar 07·3 mins

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

Here’s the crazy semaphore queue:

```
#include <pthread.h>
#include <semaphore.h>
#include <stdatomic.h>
#include <assert.h>
#include <unistd.h>
#include <stdio.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <sys/mman.h>

#define QUEUE_SIZE 8
#define NN 1024

typedef struct shq {
    int queue[QUEUE_SIZE];
    unsigned int qii; // Input index
    unsigned int qjj; // Output index
    sem_t isem; // Space
    sem_t osem; // Items
} shq;

shq* shared = 0;

int
queue_get()
{
    int rv;
    rv = sem_wait(&(shared->osem));
    assert(rv == 0);

    unsigned int ii = atomic_fetch_add(&(shared->qii), 1);
    int yy = shared->queue[ii % QUEUE_SIZE];

    rv = sem_post(&(shared->isem));
    assert(rv == 0);

    return yy;
}

void
queue_put(int xx)
{
    int rv;
    rv = sem_wait(&(shared->isem));
    assert(rv == 0);

    unsigned int jj = atomic_fetch_add(&(shared->qjj), 1);
    shared->queue[jj % QUEUE_SIZE] = xx;

    rv = sem_post(&(shared->osem));
    assert(rv == 0);
}

void
producer_proc()
{
    for (int ii = 0; ii < NN; ++ii) {
        queue_put(ii);
    }
}

int
main(int _ac, char* _av[])
{
    int kids[2];

    shared = mmap(0, 1024 * 1024, PROT_READ | PROT_WRITE,
                  MAP_SHARED | MAP_ANONYMOUS, -1, 0);

    shared->qii = 0;
    shared->qjj = 0;

    sem_init(&(shared->isem), 1, QUEUE_SIZE);
    sem_init(&(shared->osem), 1, 0);

    for (int ii = 0; ii < 2; ++ii) {
        kids[ii] = fork();
        if (kids[ii] == 0) {
            producer_proc();
            return 0;
        }
    }

    for (int ii = 0; ii < 2 * NN; ++ii) {
        int yy = queue_get();
        printf("%d\n", yy);
        usleep(1000);
    }

    for (int ii = 0; ii < 2; ++ii) {
        waitpid(kids[ii], 0, 0);
    }

    return 0;
}

```

Too much time? Let’s do more slides.

![Nat Tuck](/img/author.jpg)

Author

Nat Tuck

---

[←→

Lecture Notes: 20 Malloc Slides, pt2

2025 Mar 05](/classes/2025-01/cs4310/notes/20-more-slides/)

[Lecture Notes: 21 Advanced Malloc

2025 Mar 10
→←](/classes/2025-01/cs4310/notes/21-advanced-malloc/)

[↑](#the-top "Scroll to top")

©
2025
Nat Tuck

Powered by [Hugo](https://gohugo.io/) & [Congo](https://github.com/jpanther/congo)