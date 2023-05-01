//
// Created by matteo on 14/04/23.
//
#include <stdlib.h>
#include <stdio.h>
#include <time.h>

typedef struct {
    int type;
    float val;
    long timestamp;
} ValueStruct;

typedef struct {
    int type;
    float val[10];
    long timestamp;
} MValueStruct;

typedef struct {
    int type;
    char message[21]; // stringa null terminated lunga max 20
}  MessageStruct;

typedef struct {
    int type;
    union {
        ValueStruct val;
        MValueStruct mvals;
        MessageStruct  messages;
    };
} ExportData;

ExportData* generateRandomData(int n) {
    ExportData* res = (ExportData*)malloc(sizeof(ExportData)*n);
    for(int i=0; i<n; i++) {
        //printf("%d\n",i);
        res[i].type = 1+rand()%3;
        switch (res[i].type)
        {
            case 1:
                res[i].val.type = rand()%5+1;
                res[i].val.val = (rand()%10)/5.0;
                res[i].val.timestamp =  time(0);
                break;
            case 2:
                res[i].mvals.type = rand()%5+1;
                for(int j=0; j<10; j++)
                    res[i].mvals.val[j] = (rand()%10)/5.0;
                res[i].mvals.timestamp =  time(0);
                break;
            case 3:
                res[i].messages.type = rand()%5+1;
                int len = rand()%16+5; // lunghezza tra 5 e 20
                for(int j=0; j<len; j++)
                    res[i].messages.message[j] = (rand()%26)+'a';
                res[i].messages.message[len] = '\0';
                break;
        }
    }
    return res;
}


void export(ExportData *data, int n, FILE *pf) {
    fwrite(data, sizeof(ExportData), n, pf);
}

int main(int argc, char** argv) {
    if(argc < 2) {
        fprintf(stderr, "Not enough arguments.\n");
        exit(-1);
    }

    ExportData* data = generateRandomData(100);
    char* file_name = argv[1];
    FILE* pf = fopen(file_name, "w");

    if(pf == NULL) {
        free(data);
        fprintf(stderr, "ERROR: Could not open the file \"%s\"", file_name);
        exit(-2);
    }
    export(data, 100, pf);
    free(data);

    if(fclose(pf) == EOF) {
        fprintf(stderr, "ERROR: Could not close the file \"%s\"", file_name);
        exit(-3);
    }
    return 0;
}