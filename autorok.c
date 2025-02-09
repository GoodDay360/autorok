#include "core/head.c"
#include "core/color.c"
#include "core/get_log_file.c"
#include "core/get_runtime_file.c"


#include "module/global.c"
#include "module/json.c"
#include "module/disable.c"
#include "module/enable.c"
#include "module/get_info.c"
#include "module/overview.c"
#include "module/create.c"
#include "module/share.c"
#include "module/join.c"
#include "module/delete.c"

int main() {
    FILE *file;

    file = fopen(get_runtime_file(), "r");
    if (file) {
        fclose(file);
    } else {
        printf(BRED "[ERROR] %s does not exist.\n", get_runtime_file());
        printf(WHT "");
        return 0;
    }
    do{
        printf(BMAG "Getting information...\n"); 
        int result = get_info();
        
        if (result == 0) {
            char token[56];
            printf(WHT "[>] Enter Environment token: "); scanf("%s", &token);
            printf(MAG "Contacting zrok server...\n");
            enable(token);
            printf(WHT "\nPress any key to continue...");
            getchar(); 
            printf("\n");
            system("@cls||clear");
        }else{
            system("@cls||clear");


            int option;
            printf(BCYN "\n=== zrok auto configuration ===\n\n");
            printf(WHT "1. Disable Environment\n");
            printf(WHT "2. Overview service\n");
            printf(WHT "3. Share service\n");
            printf(WHT "4. Join service\n");
            printf(WHT "5. Create service\n");
            printf(WHT "6. Delete service\n");
            printf(WHT "7. Exit\n");

            printf(WHT "\n[>] Enter option: "); scanf("%d", &option);

            if (option == 1){
                disable();
            }else if (option == 2){
                overview();
            }else if (option == 3){
                share();
            }else if (option == 4){
                join();
            }else if (option == 5){
                create();
            }else if (option == 6){
                delete();
            }

            if (option == 7){
                printf(WHT "");
                break;
            }else if (option > 7 || option < 1){
                system("@cls||clear");
                printf(BRED "\n[ERROR] %d is not a valid option.\n", option);
            }else{
                printf(WHT "\nPress any key to go back...");
                getchar(); 
                printf("\n");
                system("@cls||clear");
            }
        }
        
    } while (1);
    printf(WHT "");
    return 0;
}
