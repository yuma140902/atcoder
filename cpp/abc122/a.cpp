#include <bits/stdc++.h>
using namespace std;

int main() {
  char c;
  cin >> c;
  if (c == 'A') {
    cout << 'T';
  } else if (c == 'T') {
    cout << 'A';
  } else if (c == 'C') {
    cout << 'G';
  } else if (c == 'G') {
    cout << 'C';
  }
  cout << endl;
}
