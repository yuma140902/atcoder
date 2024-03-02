#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int solve(char *str) {
  if (strcmp(str, "tourist") == 0) {
    return 3858;
  }
  if (strcmp(str, "ksun48") == 0) {
    return 3679;
  }
  if (strcmp(str, "Benq") == 0) {
    return 3658;
  }
  if (strcmp(str, "Um_nik") == 0) {
    return 3648;
  }
  if (strcmp(str, "apiad") == 0) {
    return 3638;
  }
  if (strcmp(str, "Stonefeang") == 0) {
    return 3630;
  }
  if (strcmp(str, "ecnerwala") == 0) {
    return 3613;
  }
  if (strcmp(str, "mnbvmar") == 0) {
    return 3555;
  }
  if (strcmp(str, "newbiedmy") == 0) {
    return 3516;
  }
  if (strcmp(str, "semiexp") == 0) {
    return 3481;
  }
  return -1;
}

int main(void) {
  char str[256];
  scanf("%s", str);

  int ans = solve(str);

  printf("%d\n", ans);

  return 0;
}

