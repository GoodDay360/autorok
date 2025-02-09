#include "../core/head.c"
#include "../core/color.c"
#include "../module/cJSON.h"


int join(){
    system("@cls||clear");
    
    printf(BMAG "Contacting zrok server...\n");
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
    char buffer[4096];
    char json_data[16384] = "";
    while (fgets(buffer, sizeof(buffer), fp) != NULL) {
        if (strstr(buffer, "unable to load environment") != NULL) {
            printf(BYEL "[?] Environment not enable.\n");
            return 0;
        } else {
            strncat(json_data, buffer, sizeof(json_data) - strlen(json_data) - 1);
        }
    }

    fclose(fp);

    
    cJSON *json_root = cJSON_Parse(json_data);
    if (json_root == NULL) {
        printf("Error parsing JSON\n");
        return 1;
    }

    // Get the "environments" array
    cJSON *environments = cJSON_GetObjectItem(json_root, "environments");
    if (environments == NULL) {
        printf("Key 'environments' not found\n");
        cJSON_Delete(json_root);
        return 1;
    }

    system("@cls||clear");
    
    // Iterate through the "environments" array
    int exist = 0;
    int env_size = cJSON_GetArraySize(environments);
    for (int i = 0; i < env_size; i++) {
        cJSON *env_item = cJSON_GetArrayItem(environments, i);
        cJSON *shares = cJSON_GetObjectItem(env_item, "shares");

        if (shares != NULL && cJSON_IsArray(shares)) {
            exist = 1;
            int ID = 0;
            printf(UCYN "%-5s %-20s %-20s %-30s %-15s %-10s\n\n", "ID", "Token", "Backend Mode", "Backend Proxy Endpoint", "Share Mode", "Reserved");
            // Iterate through the "shares" array
            int shares_size = cJSON_GetArraySize(shares);
            for (int j = 0; j < shares_size; j++) {
                ID++;
                cJSON *share_item = cJSON_GetArrayItem(shares, j);
                cJSON *backend_mode = cJSON_GetObjectItem(share_item, "backendMode");
                cJSON *backend_proxy_endpoint = cJSON_GetObjectItem(share_item, "backendProxyEndpoint");
                cJSON *reserved = cJSON_GetObjectItem(share_item, "reserved");
                cJSON *share_mode = cJSON_GetObjectItem(share_item, "shareMode");
                cJSON *token = cJSON_GetObjectItem(share_item, "token");

                if (cJSON_IsString(backend_mode) && cJSON_IsString(backend_proxy_endpoint)  && 
                    cJSON_IsBool(reserved) && cJSON_IsString(share_mode) && cJSON_IsString(token)) {
                    printf(WHT "%-5d %-20s %-20s %-30s %-15s %-10s\n", 
                        ID, token->valuestring, backend_mode->valuestring, 
                        backend_proxy_endpoint->valuestring, share_mode->valuestring, 
                        reserved->valueint ? "true" : "false"
                    );
                }
            }

            char input[1000]; 
            int selected_ID; 
            clear_input_buffer();

            printf(WHT "\n[>] Enter service ID to join | Empty to cancel: "); 
            fgets(input, sizeof(input), stdin); input[strcspn(input, "\n")] = 0; fflush(stdin);
            if (strlen(input) != 0) {
                selected_ID = atoi(input); 
                cJSON *share_item = cJSON_GetArrayItem(shares, ID - 1);
                cJSON *token = cJSON_GetObjectItem(share_item, "token");
                cJSON *backend_proxy_endpoint = cJSON_GetObjectItem(share_item, "backendProxyEndpoint");

                char is_same_bpe[5] = "y";
                printf(WHT "\n[>] Join with same backend proxy endpoint as host? (Y/n): "); 
                fgets(is_same_bpe, sizeof(is_same_bpe), stdin); is_same_bpe[strcspn(is_same_bpe, "\n")] = 0; fflush(stdin);
                toLowerCase(is_same_bpe);

                char command[512];
                if (strcmp(is_same_bpe, "y") == 0 || strcmp(is_same_bpe, "") == 0) {
                    snprintf(command, sizeof(command), "%s access private %s --bind %s", get_runtime_file(), token->valuestring, backend_proxy_endpoint->valuestring);
                }else{
                    char ip[25];
                    printf(WHT "\n[>] Enter IP address: ");scanf("%s", &ip);
                    int port;   
                    printf(WHT "[>] Enter port: ");scanf("%d", &port);
                    snprintf(command, sizeof(command), "%s access private %s --bind %s:%d", get_runtime_file(), token->valuestring, ip, port);
                }
                // Execute the command
                printf(BMAG "Contacting zrok server...\n"); printf(WHT "");

                // zrok access private mymcserverjan06 --bind 127.0.0.1:25565

                system(command);
            }else{
                printf(BYEL "[!] Action canceled. \n");
                
            }
        }
    }

    if(exist == 0) {
        fflush(stdin);
        printf(BYEL "No service found.\n");
    }
    
    // Clean up
    cJSON_Delete(json_root);
    
}