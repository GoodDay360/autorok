#include "../core/head.c"
#include "../core/color.c"


void disable(){
    system("@cls||clear");
    fflush(stdin);
    printf(BMAG "Contacting zrok server...\n");
    
    char command[512];
    snprintf(command, sizeof(command), "%s disable > %s 2>&1", get_runtime_file(), get_log_file());

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
        if (strstr(path, "zrok environment disabled") != NULL) {
            result = 1;
            break;
        } else {
            result = 0;
        }
    }
    if (result == 1) {
        printf(BYEL "[OK] The environment was successfully disabled.\n");
    }else{
        printf(BRED "[ERROR] Failed to disable Environment.\n");
    }
    fclose(fp);
    
}