#include <bits/stdc++.h>
using namespace std;

int main() {
  int A, B;
  cin >> A >> B;

  int max = A > B ? A : B;
  if (max * 2 <= 16) {
    cout << "Yay!" << endl;
  } else {
    cout << ":(" << endl;
  }

  return 0;
}
