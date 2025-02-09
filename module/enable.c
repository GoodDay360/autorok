#include "../core/head.c"
#include "../core/color.c"



void enable(char token[56]){
    char command[512];
    snprintf(command, sizeof(command), "%s enable %s > %s 2>&1", get_runtime_file(), token, get_log_file());

    // Execute the command
    system(command);
    
    FILE *fp = fopen(get_log_file(), "r");
    if (fp == NULL) {
        printf(BRED "Error opening output file.\n");
    }

    // Read the output a line at a time - output it
    int result = 0;
    char path[1035];
    while (fgets(path, sizeof(path), fp) != NULL) {
        if (strstr(path, "zrok environment was successfully enabled") != NULL) {
            result = 1;
            break;
        } else {
            result = 0;
        }
    }
    if (result == 1) {
        printf(BGRN "[OK] The environment was successfully enabled.\n");
    }else{
        system("@cls||clear");
        printf(BRED "[ERROR] Failed to enable Environment. Incorrect token or not exist.\n");
    }
    fclose(fp);
    clear_input_buffer();
}