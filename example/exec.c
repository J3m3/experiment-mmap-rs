#include <assert.h>
#include <stdio.h>
#include <fcntl.h>
#include <sys/mman.h>
#include <sys/stat.h>
#include <unistd.h>

int main(int argc, char *argv[])
{
    assert(argc >= 2);
    const char *filepath = argv[1];

    int fd = open(filepath, O_RDWR);
    assert(fd >= 0);

    struct stat statbuf;
    int err = fstat(fd, &statbuf);
    assert(err == 0);

    size_t file_size = statbuf.st_size;

    void *ptr = mmap(NULL, file_size, PROT_EXEC, MAP_PRIVATE, fd, 0);
    assert(ptr != MAP_FAILED);
    close(fd);

    ((void (*)(void))ptr)();

    err = munmap(ptr, file_size);
    assert(err == 0);

    printf("Hello from C!\n");

    return 0;
}