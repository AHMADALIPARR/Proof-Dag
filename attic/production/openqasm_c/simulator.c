/* Production kernel: OpenQASM 3 H-Rz-CNOT-Rz-CNOT state-vector simulator
#ifndef M_PI
#define M_PI 3.14159265358979323846
#endif
 * with Klong angle array input. Fully explicit, no external quantum libs.
 */
#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>
#include <complex.h>

#define N 4

typedef double complex cplx;

static void print_state(const cplx *s) {
    const char *labels[] = {"|00>", "|01>", "|10>", "|11>"};
    printf("Final state:\n");
    for (int i = 0; i < N; i++) {
        printf(" %s: % .6f %+.6fi\n", labels[i], creal(s[i]), cimag(s[i]));
    }
}

static void apply_h0(cplx *s) {
    const double inv_sqrt2 = 0.7071067811865476;
    cplx t[N];
    memcpy(t, s, sizeof(t));
    /* H ⊗ I on qubit 0 */
    s[0] = inv_sqrt2 * (t[0] + t[2]);
    s[1] = inv_sqrt2 * (t[1] + t[3]);
    s[2] = inv_sqrt2 * (t[0] - t[2]);
    s[3] = inv_sqrt2 * (t[1] - t[3]);
}

static void apply_rz(cplx *s, int qubit, double theta) {
    double half = theta / 2.0;
    cplx p0 = cexp(-I * half);
    cplx p1 = cexp( I * half);
    for (int i = 0; i < N; i++) {
        int bit = (qubit == 0) ? ((i >> 1) & 1) : (i & 1);
        s[i] *= (bit == 0) ? p0 : p1;
    }
}

static void apply_cnot(cplx *s) {
    /* control=0, target=1: swap |10> <-> |11> */
    cplx tmp = s[2];
    s[2] = s[3];
    s[3] = tmp;
}

static int *parse_klong_array(const char *filename, int *length) {
    FILE *fp = fopen(filename, "r");
    if (!fp) { perror(filename); exit(1); }
    fseek(fp, 0, SEEK_END);
    long sz = ftell(fp);
    fseek(fp, 0, SEEK_SET);
    char *buf = malloc(sz + 1);
    fread(buf, 1, sz, fp);
    buf[sz] = 0;
    fclose(fp);

    char *start = strstr(buf, ":array[");
    if (!start) { fprintf(stderr, "No :array[ found\n"); exit(1); }
    start += 7;
    char *end = strchr(start, ']');
    if (!end) { fprintf(stderr, "Missing ]\n"); exit(1); }
    *end = 0;

    int count = 0;
    for (char *p = start; *p; p++) if (*p != ' ' && (p == start || *(p-1) == ' ')) count++;
    int *arr = malloc(count * sizeof(int));
    char *tok = strtok(start, " \t");
    int i = 0;
    while (tok && i < count) {
        arr[i++] = atoi(tok);
        tok = strtok(NULL, " \t");
    }
    free(buf);
    *length = i;
    return arr;
}

int main(void) {
    int len = 0;
    int *mrad = parse_klong_array("angles.kg", &len);
    if (len < 2) { fprintf(stderr, "Need >=2 angles\n"); return 1; }

    const double PI = 3.14159265358979323846;
    double alpha = mrad[0] * (2.0 * PI) / 1000.0;
    double beta  = mrad[1] * (2.0 * PI) / 1000.0;
    free(mrad);

    cplx state[N] = {1.0 + 0.0*I, 0, 0, 0};

    apply_h0(state);
    apply_rz(state, 0, alpha);
    apply_cnot(state);
    apply_rz(state, 1, beta);
    apply_cnot(state);

    print_state(state);
    return 0;
}
