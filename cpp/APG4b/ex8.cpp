#include <bits/stdc++.h>
using namespace std;

int main() {
  int mode, price, N;
  string desc = "";

  cin >> mode;
  if (mode == 2)
    cin >> desc;
  cin >> price >> N;

  if (mode == 2)
    cout << desc << "!" << endl;
  cout << price * N << endl;
}
