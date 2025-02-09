#include "../core/head.c"
#include "../core/color.c"



char* get_log_file() {
    char cwd[MAX_PATH];
    if (GetModuleFileName(NULL, cwd, sizeof(cwd)) == 0) {
        printf("Error getting current directory.\n");
        return NULL;
    }

    // Allocate memory for log_file on the heap
    char *log_file = malloc(256 * sizeof(char));
    if (log_file == NULL) {
        printf("Error allocating memory.\n");
        return NULL;
    }

    // Extract the directory path
    char *last_backslash = strrchr(cwd, '\\');
    if (last_backslash != NULL) {
        *last_backslash = '\0';
    }

    snprintf(log_file, 256, "%s\\log.txt", cwd);

    return log_file;
}