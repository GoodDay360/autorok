#include "../core/head.c"
#include "../core/color.c"

char* get_runtime_file() {
    char cwd[MAX_PATH];
    if (GetModuleFileName(NULL, cwd, sizeof(cwd)) == 0) {
        printf("Error getting current directory.\n");
        return NULL;
    }

    // Allocate memory for runtime on the heap
    char *runtime = malloc(512 * sizeof(char));
    if (runtime == NULL) {
        printf("Error allocating memory.\n");
        return NULL;
    }

    // Extract the directory path
    char *last_backslash = strrchr(cwd, '\\');
    if (last_backslash != NULL) {
        *last_backslash = '\0';
    }

    snprintf(runtime, 512, "%s\\runtime.exe", cwd);

    return runtime;
}