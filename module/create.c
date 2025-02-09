#include "../core/head.c"
#include "../core/color.c"


int create(){
    system("@cls||clear");
    char BACKEND_MODE[6][25] = {"proxy", "drive", "web", "caddy", "tcpTunnel", "udpTunnel"};

    int selected_backend_mode;
    printf(BMAG "\n=== Select Backend Mode ===\n");
    printf(WHT "1. Proxy\n");
    printf(WHT "2. Drive\n");
    printf(WHT "3. Web\n");
    printf(WHT "4. Caddy\n");
    printf(WHT "5. TCP Tunnel\n");
    printf(WHT "6. UDP Tunnel\n");
    printf(WHT "[>] Enter backend mode: "); scanf("%d", &selected_backend_mode);

    char SHARE_MODE[2][25] = {"public", "private"};
    int selected_share_mode;
    printf(BMAG "\n=== Select Share Mode ===\n");
    printf(WHT "1. Public\n");
    printf(WHT "2. Private\n");
    printf(WHT "[>] Enter share mode: "); scanf("%d", &selected_share_mode);
    printf("\n");
    
    char ip[25];
    printf(WHT "[>] Enter IP address: "); scanf("%s", &ip);

    int port;
    printf(WHT "[>] Enter port: "); scanf("%d", &port);

    char name[25];
    printf(WHT "[>] Enter name | Empty for random: "); 
    clear_input_buffer(); fgets(name, sizeof(name), stdin); name[strcspn(name, "\n")] = 0;
    fflush(stdin);

    char command[512];
    if (strlen(name) != 0){
        snprintf(command, sizeof(command), 
            "%s reserve %s %s:%d --backend-mode %s --unique-name %s > %s 2>&1",
            get_runtime_file(), 
            SHARE_MODE[selected_share_mode - 1], ip, port, BACKEND_MODE[selected_backend_mode - 1], name,
            get_log_file()
        );

    }else{
        snprintf(command, sizeof(command), 
            "%s reserve %s %s:%d --backend-mode %s > %s 2>&1",
            get_runtime_file(), 
            SHARE_MODE[selected_share_mode - 1], ip, port, BACKEND_MODE[selected_backend_mode - 1],
            get_log_file()
        );
    }
    
    // Execute the command
    system(command);

    // Open the temporary file for reading
    FILE *fp = fopen(get_log_file(), "r");
    if (fp == NULL) {
        printf(BRED "Error opening output file.\n");
        return 0;
    }

    char buffer[4096];
    char json_data[16384] = "";
    while (fgets(buffer, sizeof(buffer), fp) != NULL) {
        strncat(json_data, buffer, sizeof(json_data) - strlen(json_data) - 1);
    }
    fclose(fp);

    printf(BGRN "[OK] Service successfully created.\n");
    printf(BWHT "[MSG] %s\n", json_get(json_data, "msg"));
    printf(BWHT "[FILE] %s\n", json_get(json_data, "file"));
    printf(BWHT "[FUNC] %s\n", json_get(json_data, "func"));
    printf(BWHT "[TIME] %s\n", json_get(json_data, "time"));
    
    return 0;

}