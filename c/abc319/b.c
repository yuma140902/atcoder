#include <stdio.h>
#include <stdlib.h>

int main(void) {
  int N;
  char s[1002];
  scanf("%d", &N);

  for (int i = 0; i <= N; ++i) {
    s[i] = '-';
    for (int j = 9; j >= 1; --j) {
      if (N % j == 0) {
        if (i % (N / j) == 0) {
          s[i] = '0' + j;
        }
      }
    }
  }
  s[N + 1] = 0;

  puts(s);
}

