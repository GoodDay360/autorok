#include "../core/head.c"
#include "../core/color.c"

char *my_strndup(const char *s, size_t n) {
    char *result;
    size_t len = strnlen(s, n);
    result = (char *)malloc(len + 1);
    if (!result) return NULL;
    result[len] = '\0';
    return (char *)memcpy(result, s, len);
}


char* json_get(const char *json_data, const char *key) {
    char *key_pos = strstr(json_data, key);
    if (!key_pos) return NULL;

    char *value_start = strchr(key_pos, ':');
    if (!value_start) return NULL;
    value_start++;

    // Skip any whitespace
    while (*value_start == ' ' || *value_start == '\t' || *value_start == '\n') {
        value_start++;
    }

    char *value_end;
    if (*value_start == '[') {
        // Handle array
        value_end = strchr(value_start, ']');
        if (!value_end) return NULL;
        value_end++;
    } else if (*value_start == '"') {
        // Handle string
        value_start++;
        value_end = strchr(value_start, '"');
        if (!value_end) return NULL;
    } else if (strncmp(value_start, "true", 4) == 0 || strncmp(value_start, "false", 5) == 0) {
        // Handle boolean
        value_end = value_start;
        while (*value_end && *value_end != ',' && *value_end != '}' && *value_end != ' ' && *value_end != '\t' && *value_end != '\n') {
            value_end++;
        }
    } else {
        // Handle number or other types
        value_end = value_start;
        while (*value_end && *value_end != ',' && *value_end != '}' && *value_end != ' ' && *value_end != '\t' && *value_end != '\n') {
            value_end++;
        }
    }

    size_t value_length = value_end - value_start;
    char *value = (char*)malloc(value_length + 1);
    if (!value) return NULL;

    strncpy(value, value_start, value_length);
    value[value_length] = '\0';

    // Remove quotes if present
    char *src = value, *dst = value;
    while (*src) {
        if (*src != '"' && *src != '\'') {
            *dst++ = *src;
        }
        src++;
    }
    *dst = '\0';

    // Handle boolean values
    if (strcmp(value, "true") == 0 || strcmp(value, "false") == 0) {
        return value;
    }

    return value;
}



void json_array_iterate(const char *json_string, void (*callback)(const char *)) {
    const char *pos = json_string;
    while ((pos = strstr(pos, "{")) != NULL || (pos = strstr(pos, "[")) != NULL) {
        const char *end = NULL;
        if (*pos == '{') {
            end = strstr(pos, "}");
        } else if (*pos == '[') {
            end = strstr(pos, "]");
        }
        if (end == NULL) {
            break;
        }

        char *item = my_strndup(pos, end - pos + 1);
        if (item == NULL) {
            printf("Memory allocation error\n");
            return;
        }

        callback(item);

        free(item);
        pos = end + 1;
    }
}


char** json_create_array_from_child_key(const char* json_data, const char* key, size_t* array_size) {
    char** result_array = NULL;
    *array_size = 0;

    const char* pos = json_data;
    char key_pattern[256];
    snprintf(key_pattern, sizeof(key_pattern), "\"%s\":", key);

    while ((pos = strstr(pos, key_pattern)) != NULL) {
        pos += strlen(key_pattern);
        while (*pos == ' ' || *pos == '\"' || *pos == '\'') pos++;
        const char* end = strchr(pos, ',');
        if (!end) end = strchr(pos, '}');
        if (end) {
            size_t len = end - pos;
            result_array = realloc(result_array, (*array_size + 1) * sizeof(char*));
            result_array[*array_size] = malloc(len + 1);
            strncpy(result_array[*array_size], pos, len);
            result_array[*array_size][len] = '\0';

            // Remove any remaining quotes from the extracted value
            for (size_t i = 0; i < len; i++) {
                if (result_array[*array_size][i] == '\"' || result_array[*array_size][i] == '\'') {
                    memmove(&result_array[*array_size][i], &result_array[*array_size][i + 1], len - i);
                    len--;
                    i--;
                }
            }

            (*array_size)++;
            pos = end + 1;
        }
    }

    return result_array;
}