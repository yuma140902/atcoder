#include <bits/stdc++.h>
using namespace std;
typedef int64_t ll;
const int INF = 900000;
const ll INFL = 9000000;

#define rep(i, n) for(int i=0; i < (n); ++i)
#define per(i, n) for(int i=(n)-1; i >= 0; --i)
#define range(i, begin, end) for(int i=(begin); i < (end); ++i)

#define ALL(vec) vec.begin(), vec.end()


void solve(std::string s){
  cout << s.replace(5, 1, " ").replace(13, 1, " ") << endl;
}

int main(){
  std::string s;
		std::cin >> s;
  solve(s);
  return 0;
}

