#include <stdio.h>
#include <fcntl.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/socket.h>
#ifdef __linux__
#include <sys/sendfile.h>
#endif /* __linux__ */

int tcp_utils_send_file(char* file_path, long file_size, int socket)
{
    printf("File path: %s\n", file_path);
    printf("File size: %ld\n", file_size);
    printf("Socket FD: %d\n", socket);
    int resource_file = open(file_path, O_RDONLY);
    if (resource_file == -1)
    {
        printf("Error opening file %s\n", file_path);
        return -1;
    }
#ifdef __linux__
    ssize_t bytes_sent = sendfile(socket, resource_file, NULL, file_size);
    printf("Size `%ld` bytes.\n", file_size);
    printf("Sent `%ld` bytes.\n", bytes_sent);
    if (bytes_sent == -1)
#else
    off_t len = file_size; // set to 0 will send all the origin file
    int res   = sendfile(resource_file, socket, 0, &len, NULL, 0);
    printf("Sent `%lld` bytes.\n", len);
    if (res == -1)
#endif
    {
        printf("Failed to send file\n");
        close(resource_file);
        return -2;
    }
    close(resource_file);
    return 0;
}
