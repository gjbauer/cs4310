Lecture Notes: 39 Nonblocking I/O · Classes w/ Prof. Nat Tuck



[↓Skip to main content](#main-content)

[Classes w/ Prof. Nat Tuck](/)

* [Home](/)
* [cs2370](/classes/2025-01/cs2370/)
* [cs2470](/classes/2025-01/cs2470/)
* [cs4310](/classes/2025-01/cs4310/)
* [Inkfish](https://inkfish.homework.quest/)

Lecture Notes: 39 Nonblocking I/O
=================================

2025 Apr 30·Updated: 2025 May 01·5 mins

Bad news: There was a typo in the Project 2 starter code that broke
the assignment for many students. I’ve posted a new starter code tarball that
fixes the issue and extended the due date until next Monday.

Today: Non-blocking I/O

Motivation:

* Concurrency is important.
* Sometimes processes or even threads are a heavier weight tool than we
  need, because we need concurrency but not parallel execution.
* This is common for I/O bound tasks. A simple webserver needs to handle multiple
  requests concurrently, but it tends to spend its time waiting network messages
  and disk I/O, rather than doing enough computation to require multiple CPU
  cores.

Simple network server:

```
#include <arpa/inet.h>
#include <netinet/in.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <unistd.h>

#define PORT 9090
#define BUFFER_SIZE 1024

int
main(int argc, char* argv[])
{
    int server_fd, client_fd;
    struct sockaddr_in server_addr, client_addr;
    socklen_t client_len = sizeof(client_addr);
    char buffer[BUFFER_SIZE];
    int opt = 1;

    // Create socket
    if ((server_fd = socket(AF_INET, SOCK_STREAM, 0)) < 0) {
        perror("Socket creation failed");
        exit(1);
    }

    // Set socket options to reuse address and port
    if (setsockopt(server_fd, SOL_SOCKET, SO_REUSEADDR, &opt, sizeof(opt))) {
        perror("setsockopt failed");
        exit(1);
    }

    // Configure server address
    memset(&server_addr, 0, sizeof(server_addr));
    server_addr.sin_family = AF_INET;
    server_addr.sin_addr.s_addr = INADDR_ANY;
    server_addr.sin_port = htons(PORT);

    // Bind socket to address and port
    if (bind(server_fd, (struct sockaddr*)&server_addr, sizeof(server_addr)) < 0) {
        perror("Bind failed");
        exit(1);
    }

    // Listen for connections
    if (listen(server_fd, 5) < 0) {
        perror("Listen failed");
        exit(1);
    }

    printf("TCP Echo Server listening on port %d\n", PORT);

    while (1) {
        // Accept incoming connection
        if ((client_fd = accept(server_fd, (struct sockaddr*)&client_addr, &client_len)) < 0) {
            perror("Accept failed");
            continue;
        }

        printf("Connection accepted from %s:%d\n",
            inet_ntoa(client_addr.sin_addr), ntohs(client_addr.sin_port));

        // Echo back received data
        int bytes_read;
        while ((bytes_read = read(client_fd, buffer, BUFFER_SIZE - 1)) > 0) {
            buffer[bytes_read] = '\0';
            printf("Received: %s", buffer);

            // Echo back
            write(client_fd, buffer, bytes_read);

            // If the message ends with a newline, we consider it a complete line
            if (buffer[bytes_read - 1] == '\n') {
                printf("Line echoed back\n");
            }
        }

        if (bytes_read < 0) {
            perror("Read error");
            exit(1);
        }

        printf("Connection closed\n");
        close(client_fd);
    }

    close(server_fd);
    return 0;
}

```

```
$ nc localhost 9090

```

Problem:

* If two clients connect to this server at the same time,
  the second client won’t get any responses until the first
  client disconnects.
* This looks hard to avoid: the I/O syscalls like read and accept
  are blocking by default. They’ll stop the whole process until
  something happens.

Solution part 1:

* We can pass the O\_NONBLOCK flag to set our file descriptors as
  non-blocking.
* This makes them return immediately if there’s nothing to do with
  an EWOULDBLOCK error.

New problem:

* IO blocks for a reason: So the program doesn’t spin and saturate a whole
  CPU polling for IO events in a loop.

Solution:

* We need a way to block until something happens.
* Linux provides a couple of syscalls that do this. Today we’ll use
  select(2).

Network server with non-blocking I/O:

```
#define _GNU_SOURCE
#include <arpa/inet.h>
#include <assert.h>
#include <errno.h>
#include <netinet/in.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <unistd.h>

#define PORT 9090
#define BUFFER_SIZE 1024
#define MAX_CLIENTS 10

typedef struct conn_state {
    int fd;
    char buffer[BUFFER_SIZE];
} conn_state;

conn_state state[MAX_CLIENTS];

void
accept_conn(int server_fd)
{
    int client_fd;
    struct sockaddr_in client_addr;
    socklen_t client_len = sizeof(client_addr);

    client_fd = accept4(server_fd,
        (struct sockaddr*)&client_addr,
        &client_len,
        SOCK_NONBLOCK);
    if (client_fd < 0) {
        if (errno != EWOULDBLOCK) {
            perror("Accept failed");
        }
        return;
    }

    printf("Connection accepted from %s:%d\n",
        inet_ntoa(client_addr.sin_addr), ntohs(client_addr.sin_port));

    for (int ii = 0; ii < MAX_CLIENTS; ++ii) {
        if (state[ii].fd == -1) {
            state[ii].fd = client_fd;
            memset(state[ii].buffer, 0, BUFFER_SIZE);
            return;
        }
    }

    fprintf(stderr, "Too many clients.\n");
    close(client_fd);
}

void
print_lines(int fd, char* buffer)
{
    int ii = 0;
    for (; ii < BUFFER_SIZE; ++ii) {
        if (buffer[ii] == 0) {
            return;
        }
        if (buffer[ii] == '\n') {
            ii++;
            break;
        }
    }

    write(fd, buffer, ii);

    char temp[BUFFER_SIZE];
    memset(temp, 0, BUFFER_SIZE);
    memcpy(temp, buffer + ii, BUFFER_SIZE - ii);
    memcpy(buffer, temp, BUFFER_SIZE);

    print_lines(fd, buffer);
}

void
read_data(int fd)
{
    char* buffer = 0;

    for (int ii = 0; ii < MAX_CLIENTS; ++ii) {
        if (state[ii].fd == fd) {
            buffer = state[ii].buffer;
            break;
        }
    }

    assert(buffer != 0);

    int bytes_read;
    while ((bytes_read = read(fd, buffer, BUFFER_SIZE - 1)) > 0) {
        buffer[bytes_read] = '\0';
        printf("Received: %s", buffer);

        print_lines(fd, buffer);
    }

    if (bytes_read == 0) {
        printf("Connection closed\n");
        close(fd);
        for (int ii = 0; ii < MAX_CLIENTS; ++ii) {
            if (state[ii].fd == fd) {
                state[ii].fd = -1;
            }
        }
    }

    if (bytes_read < 0) {
        if (errno == EWOULDBLOCK) {
            return;
        }

        perror("Read error");
        exit(1);
    }
}

int
main(int argc, char* argv[])
{
    int server_fd;
    struct sockaddr_in server_addr;
    char buffer[BUFFER_SIZE];
    int opt = 1;
    fd_set fds;

    for (int ii = 0; ii < MAX_CLIENTS; ++ii) {
        state[ii].fd = -1;
    }

    // NOTE: Add SOCK_NONBLOCK
    if ((server_fd = socket(AF_INET, SOCK_STREAM | SOCK_NONBLOCK, 0)) < 0) {
        perror("Socket creation failed");
        exit(1);
    }

    if (setsockopt(server_fd, SOL_SOCKET, SO_REUSEADDR, &opt, sizeof(opt))) {
        perror("setsockopt failed");
        exit(1);
    }

    memset(&server_addr, 0, sizeof(server_addr));
    server_addr.sin_family = AF_INET;
    server_addr.sin_addr.s_addr = INADDR_ANY;
    server_addr.sin_port = htons(PORT);

    if (bind(server_fd, (struct sockaddr*)&server_addr, sizeof(server_addr)) < 0) {
        perror("Bind failed");
        exit(1);
    }

    if (listen(server_fd, 5) < 0) {
        perror("Listen failed");
        exit(1);
    }

    printf("TCP Echo Server listening on port %d\n", PORT);

    while (1) {
        // Figure out which file descriptors we're dealing with.
        FD_ZERO(&fds);
        FD_SET(server_fd, &fds);
        int max_fd = server_fd;
        for (int ii = 0; ii < MAX_CLIENTS; ++ii) {
            if (state[ii].fd != -1) {
                FD_SET(state[ii].fd, &fds);
                if (state[ii].fd > max_fd) {
                    max_fd = state[ii].fd;
                }
            }
        }

        struct timeval timeout;
        timeout.tv_sec = 1;
        timeout.tv_usec = 0;
        int rv = select(max_fd + 1, &fds, 0, 0, &timeout);
        if (rv < 0) {
            perror("select");
            exit(1);
        }
        if (rv > 0) {
            if (FD_ISSET(server_fd, &fds)) {
                accept_conn(server_fd);
            }
            for (int ii = 0; ii < MAX_CLIENTS; ++ii) {
                if (FD_ISSET(state[ii].fd, &fds)) {
                    read_data(state[ii].fd);
                }
            }
        }
    }

    close(server_fd);
    return 0;
}

```

![Nat Tuck](/img/author.jpg)

Author

Nat Tuck

---

[←→

Lecture Notes: 38 Semester Summary

2025 Apr 28](/classes/2025-01/cs4310/notes/38-summary/)

[↑](#the-top "Scroll to top")

©
2025
Nat Tuck

Powered by [Hugo](https://gohugo.io/) & [Congo](https://github.com/jpanther/congo)