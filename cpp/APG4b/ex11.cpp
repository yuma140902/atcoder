#include <bits/stdc++.h>
using namespace std;

int main() {
  int n, a, b;
  char op;

  cin >> n;
  cin >> a;

  for (int i = 0; i < n; ++i) {
    cin >> op >> b;
    switch (op) {
    case '+':
      a += b;
      break;
    case '-':
      a -= b;
      break;
    case '*':
      a *= b;
      break;
    case '/':
      if (b != 0) {
        a /= b;
      } else {
        cout << "error";
        goto end;
      }
      break;
    }
    cout << (i + 1) << ":" << a << endl;
  }
end:
  return 0;
}
