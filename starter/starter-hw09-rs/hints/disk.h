// based on cs3650 starter code

#ifndef PAGES_H
#define PAGES_H
#include <stdio.h>

#include <stdlib.h>
#include <stddef.h>

void pages_init(const char* path);
void storage_init();
void pages_free();
void* pages_get_page(int pnum);
void* get_pages_bitmap();
void* get_inode_bitmap();
void* get_inode_start();
void* get_root_start();
int alloc_page();
void free_page(int pnum);

char* read_d(size_t size, size_t offset);

void write_d(char *buf, size_t size, size_t offset);

size_t bitmap_get(void* bm, size_t ii);
void bitmap_put(void* bm, size_t ii, size_t vv);

size_t inode_bitmap_get(size_t ii);
void inode_bitmap_put(size_t ii, size_t vv);

#endif
