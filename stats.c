#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static int cmp_int(const void *a, const void *b) {
    int x = *(const int *)a;
    int y = *(const int *)b;
    return (x > y) - (x < y);
}

double mean(const int *arr, int n) {
    long long sum = 0;
    for (int i = 0; i < n; i++) sum += arr[i];
    return (double)sum / (double)n;
}

double median(int *arr, int n) {
    qsort(arr, n, sizeof(int), cmp_int);
    if (n % 2 == 1) {
        return arr[n / 2];
    } else {
        int a = arr[(n / 2) - 1];
        int b = arr[n / 2];
        return (a + b) / 2.0;
    }
}

int *mode(const int *sorted, int n, int *outCount) {
    int maxFreq = 1;
    int currFreq = 1;

    for (int i = 1; i < n; i++) {
        if (sorted[i] == sorted[i - 1]) currFreq++;
        else {
            if (currFreq > maxFreq) maxFreq = currFreq;
            currFreq = 1;
        }
    }
    if (currFreq > maxFreq) maxFreq = currFreq;

    if (maxFreq == 1) {
        *outCount = 0;
        return NULL;
    }

    int *modes = malloc(n * sizeof(int));
    if (!modes) {
        perror("malloc");
        exit(1);
    }

    int modeCount = 0;
    currFreq = 1;

    for (int i = 1; i < n; i++) {
        if (sorted[i] == sorted[i - 1]) currFreq++;
        else {
            if (currFreq == maxFreq) {
                modes[modeCount++] = sorted[i - 1];
            }
            currFreq = 1;
        }
    }
    if (currFreq == maxFreq) {
        modes[modeCount++] = sorted[n - 1];
    }

    *outCount = modeCount;
    return modes;
}

int main(int argc, char *argv[]) {
    if (argc < 2) {
        printf("Usage: %s <int> <int> ...\n", argv[0]);
        return 0;
    }

    int n = argc - 1;
    int *arr = malloc(n * sizeof(int));
    if (!arr) {
        perror("malloc");
        return 1;
    }

    for (int i = 0; i < n; i++) {
        arr[i] = atoi(argv[i + 1]);
    }

    int *arrForMedian = malloc(n * sizeof(int));
    int *arrForMode = malloc(n * sizeof(int));
    if (!arrForMedian || !arrForMode) {
        perror("malloc");
        free(arr);
        free(arrForMedian);
        free(arrForMode);
        return 1;
    }

    memcpy(arrForMedian, arr, n * sizeof(int));
    memcpy(arrForMode, arr, n * sizeof(int));
    qsort(arrForMode, n, sizeof(int), cmp_int);

    double m = mean(arr, n);
    double med = median(arrForMedian, n);

    int modeCount = 0;
    int *modes = mode(arrForMode, n, &modeCount);

    printf("Count: %d\n", n);
    printf("Mean: %.3f\n", m);
    printf("Median: %.3f\n", med);

    if (modeCount == 0) {
        printf("Mode: none\n");
    } else {
        printf("Mode(s): ");
        for (int i = 0; i < modeCount; i++) {
            printf("%d%s", modes[i], (i == modeCount - 1) ? "" : ", ");
        }
        printf(" (frequency=%d)\n", 0);
    }

    free(arr);
    free(arrForMedian);
    free(arrForMode);
    free(modes);

    return 0;
}