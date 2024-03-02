#include <bits/stdc++.h>
using namespace std;
typedef int64_t ll;
const int INF = 900000;
const ll INFL = 9000000;

#define rep(i, n) for(int i=0; i < (n); ++i)
#define per(i, n) for(int i=(n)-1; i >= 0; --i)
#define range(i, begin, end) for(int i=(begin); i < (end); ++i)

#define ALL(vec) vec.begin(), vec.end()


void solve(long long K, long long S){
  ll cnt = 0;
  rep(x, K+1) {
    rep(y, K+1) {
      int z = S - x - y;
      if(0 <= z && z <= K) ++cnt;
    }
  }
  cout << cnt << endl;
}

int main(){
  long long K;
		scanf("%lld",&K);
		long long S;
		scanf("%lld",&S);
  solve(K, S);
  return 0;
}

