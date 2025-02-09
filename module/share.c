#include "../core/head.c"
#include "../core/color.c"
#include "../module/cJSON.h"


int share(){
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

            printf(WHT "\n[>] Enter service ID to share | Empty to cancel: "); 
            fgets(input, sizeof(input), stdin); input[strcspn(input, "\n")] = 0; 
            fflush(stdin);
            if (strlen(input) != 0) {
                selected_ID = atoi(input); 
                cJSON *share_item = cJSON_GetArrayItem(shares, ID - 1);
                cJSON *token = cJSON_GetObjectItem(share_item, "token");
                char command[512];
                snprintf(command, sizeof(command), "%s share reserved %s ", get_runtime_file(), token->valuestring);

                // Execute the command
                printf(BMAG "Contacting zrok server...\n"); printf(WHT "");
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