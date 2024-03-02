#include <bits/stdc++.h>
using namespace std;
typedef int64_t ll;
const int INF = 900000;
const ll INFL = 9000000;

#define rep(i, n) for(int i=0; i < (n); ++i)
#define per(i, n) for(int i=(n)-1; i >= 0; --i)
#define range(i, begin, end) for(int i=(begin); i < (end); ++i)

#define ALL(vec) vec.begin(), vec.end()


void solve(long long N, std::string S){
  int lvl = 0;
  int start = 0;
  rep(i, N){
    if(S[i] == '('){
      ++lvl;
    }
    else{
      --lvl;
    }

    if(lvl == -1) {
      ++start;
      lvl = 0;
    }
  }
  rep(i, lvl) S.push_back(')');
  rep(i, start) cout << '(';
  cout << S << endl;

}

int main(){
  long long N;
		scanf("%lld",&N);
		std::string S;
		std::cin >> S;
  solve(N, S);
  return 0;
}

