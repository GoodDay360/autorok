#include "../core/head.c"
#include "../core/color.c"




int get_info(){

    // Run the command and redirect output to a temporary file
    char command[512];
    snprintf(command, sizeof(command), "%s overview > %s 2>&1", get_runtime_file(), get_log_file());

    // Execute the command
    system(command);

    // Open the temporary file for reading
    FILE *fp = fopen(get_log_file(), "r");
    if (fp == NULL) {
        printf(BRED "Error opening output file.\n");
        return 0;
    }

    // Read the output a line at a time - output it
    int result = 0;
    char path[1035];
    while (fgets(path, sizeof(path), fp) != NULL) {
        if (strstr(path, "unable to load environment") != NULL) {
            result = 0;
        } else {
            result = 1;
        }
    }
    fclose(fp);

    if (result == 1) {
        return 1;
    }else{
        system("@cls||clear");
        printf(BYEL "[?] Environment not enable.\n");
        return 0;
    }
}