#include <bits/stdc++.h>
using namespace std;
typedef int64_t ll;
const int INF = 900000;
const ll INFL = 9000000;

#define rep(i, n) for(int i=0; i < (n); ++i)
#define per(i, n) for(int i=(n)-1; i >= 0; --i)
#define range(i, begin, end) for(int i=(begin); i < (end); ++i)

#define ALL(vec) vec.begin(), vec.end()


void solve(char c){
  switch(c) {
    case 'a':
    case 'e':
    case 'i':
    case 'o':
    case 'u':
      cout << "vowel" << endl;
      break;
    default:
      cout << "consonant" << endl;
  }
}

int main(){
  char c;
	std::cin >> c;
  solve(c);
  return 0;
}

