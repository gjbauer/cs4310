Lecture Notes: 36 More Concurrency · Classes w/ Prof. Nat Tuck



[↓Skip to main content](#main-content)

[Classes w/ Prof. Nat Tuck](/)

* [Home](/)
* [cs2370](/classes/2025-01/cs2370/)
* [cs2470](/classes/2025-01/cs2470/)
* [cs4310](/classes/2025-01/cs4310/)
* [Inkfish](https://inkfish.homework.quest/)

Lecture Notes: 36 More Concurrency
==================================

2025 Apr 21·Updated: 2025 Apr 22·5 mins

Then [Concurrency Demos](https://github.com/NatTuck/scratch-2025-01/tree/main/4310/35/35-concurrency)

Review: Conditions for a Data Race:

1. Multiple threads
2. Shared data
3. At least one thread writes to the shared data

Plan C: Make Shared Data Immutable [#](#plan-c-make-shared-data-immutable)
--------------------------------------------------------------------------

This eliminates condition 3, writing to the shared data.

Example Language: Rust

Example: rustsum/src/main.rs

```
use std::thread;
use std::sync::Arc;

fn main() {
    let top = 100 * 1000 * 1000;
    let xs : Arc<Vec<i64>> = Arc::new((0..top).collect());

    let each = top / 10;
    let mut kids = vec![];
    for ii in 0..10 {
        let local_xs = xs.clone();
        let local_each = each.clone();

        kids.push(thread::spawn(move || {
            let i0 = local_each * ii;
            let i1 = i0 + local_each;

            let mut s = 0;
            for jj in i0..i1 {
                s += local_xs[jj as usize];
            }

            s
        }));
    }

    let mut sum = 0;
    for tt in kids {
        sum += tt.join().unwrap();
    }

    //let y = xs.iter().fold(0, |acc, x| acc + x);
    println!("sum = {}", sum);
}

```

Plan D: Ban Everything [#](#plan-d-ban-everything)
--------------------------------------------------

Example Platform: Erlang (aka Elixir)

We can eliminate data races entirely by making data all data immutable. Once an
object is created, it cannot be changed.

Erlang programs are structured as a collection of lightweight “processes”.
Communication between processes is by message passing. Because data is
immutable, it’s safe to pass pointers to shared data as messages - although
Erlang can also be run distributed across multiple machines, in which case
messages are copied.

In the distributed context, mutation doesn’t really make sense anyway.
Mutating the local copy can’t effect a remote copy of the “same” object.

This model is great for concurrency, and great for executing concurrently
structured programs in parallel for a speedup. It’s not the greatest for
parallel speedup though - Erlang runs in an interpreter, and mutation tends to
be pretty good for fast computation.

Erlang’s main design goal is reliability. If some piece of the system crashes,
another piece (potentially on another machine) can notice and restart it.

Example: elixir / demo.ex - startPrinter, startSender

```
# Promises
#  Demo.mapFib   vs.    Demo.pmapFib

# Messages
#  {:ok, pid} = Demo.startPrinter
#  send(pid, "Hello")
#  Demo.startSender(pid)

defmodule Demo do
  def fib(x) when x < 2 do
    1
  end

  def fib(x) do
    fib(x - 1) + fib(x - 2)
  end

  def mapFib() do
    {t, v} = :timer.tc(&Enum.map/2, [[38, 38, 38, 38], &Demo.fib/1])
    IO.puts "Time: #{t/1000000.0}s"
    v
  end

  def pmap(xs, op) do
    Enum.map(xs, fn x -> Task.async(fn -> op.(x) end) end)
    |> Enum.map(&Task.await/1)
  end

  def pmapFib() do
    {t, v} = :timer.tc(&Demo.pmap/2, [[38, 38, 38, 38], &Demo.fib/1])
    IO.puts "Time: #{t/1000000.0}s"
    v
  end

  def printer() do
    receive do
      {:say, msg} -> IO.puts msg
      other       -> IO.puts "Huh? #{other}"
    end
    printer()
  end

  def startPrinter() do
    Task.start(&printer/0)
  end

  def sender(pid, x) do
    send(pid, {:say, x})
    Process.sleep(1000)
    sender(pid, x + 1)
  end

  def startSender(pid) do
    Task.start(Demo, :sender, [pid, 0])
  end
end

```

Plan E: Accept Races, Fix Later with Transactional Memory [#](#plan-e-accept-races-fix-later-with-transactional-memory)
-----------------------------------------------------------------------------------------------------------------------

Example Langauge: Clojure, a LISP on the JVM.

Like Erlang, it takes the immutability path to deal with concurrency, but
instead of message passing it has a concept of “refs”, which are mutable
references to immutable data.

Refs can be updated transactionally. Rather than avoiding data races,
transactions detect them and roll back / replay any transaction that ran on old
data.

Transaction Advantages:

* No data corruption from data races
* No deadlock
* No mutual exclusion needed for values that aren’t written to during a transaction.

Transaction Disadvantages:

* Need to handle rollbacks / replays. If transactions have side effects, those
  side effects may happen multiple times.
* Slow transactions can be delayed pretty much forever by faster transactions
  that invalidate their inputs.

Transactions are the same strategy that databases use for concurrent updates.

Example: clojure / tmem.clj

```
(def n (ref 0))

(defn incn []
  (dosync
   (alter n inc)))

(dorun (dosync (apply pcalls (repeat 10000 incn))))

(println @n)

(shutdown-agents)

```

Another Approach: Data Parallelism [#](#another-approach-data-parallelism)
--------------------------------------------------------------------------

Example Technology: OpenCL

OpenCL is a programming system for building programs that run on graphics
cards. Graphics cards, or GPUs, are a bit different from regular CPUs. Rather
than having one processor with a couple cores, they have a bunch of
“processors”, each with hundreds of “shader units”. A shader unit is basically
a single vector ALU - something that can execute arithmetic instructions on
4 or so values in parallel.

On a GPU, it’s perfectly reasonable to plan to execute 2000 additions in
parallel in one clock cycle.

The trick is that GPUs really like to perform the *same operation* in parallel.
In fact, each individual processor can generally only load one program to run
on its hundreds of shader units.

So instead of the basic addition operation adding together two numbers, on a
GPU it adds together two arrays. The arrays generally represent mathematical
vectors or matrices, but that’s just a mental model. Anything where you want
to operate on entire arrays at once will work great on a GPU.

This programming model of performing the same operation in parallel on many
different values (elements of the array) is called data parallelism. It’s
required on GPUs, but it’s common on supercomputers too. When you have a
cluster of 1000 PCs, it’s easier to think about them working together on one
array calculation than to reason about them individually.

```
; clojure -cp .
; (require 'demo)
; (time (fib 40))

(defn fib [x]
  (if (< x 2)
    1
    (+ (fib (- x 1))
       (fib (- x 2)))))

(def nums (into [] (repeat 5 38)))

; (time (map fib nums))
; (time (pmap fib nums))

```

![Nat Tuck](/img/author.jpg)

Author

Nat Tuck

---

[←→

Lecture Notes: 35 Concurrency

2025 Apr 17](/classes/2025-01/cs4310/notes/35-concurrency/)

[Lecture Notes: 37 Exploit Prevention

2025 Apr 23
→←](/classes/2025-01/cs4310/notes/37-exploit-prevention/)

[↑](#the-top "Scroll to top")

©
2025
Nat Tuck

Powered by [Hugo](https://gohugo.io/) & [Congo](https://github.com/jpanther/congo)