# Epoll: A look at how mio works

First implementation of Event Queue

``` Type of IO```

- Blocking IO
- Non-Blocking IO

Always thought all IO is blocking until I saw the type of IO.
Blocking IO: The OS park your thread until the IO is done.
Non-Blocking IO: Instead of parking your thread, the OS return immediately and notify you when the IO is done.

Are all IO Blocking? The Million Dollar Question.
The answer is Maybe. Look at this statement here.

"Finally, a question that’s easy to answer. The answer is a big, resounding… maybe. The thing is that
not all I/O operations will block in the sense that the operating system will park the calling thread and
it will be more efficient to switch to another task. The reason for this is that the operating system is
smart and will cache a lot of information in memory. If information is in the cache, a syscall requesting
that information would simply return immediately with the data, so forcing a context switch or any
rescheduling of the current task might be less efficient than just handling the data synchronously.
The problem is that there is no way to know for sure whether I/O is blocking and it depends on what
you’re doing."


