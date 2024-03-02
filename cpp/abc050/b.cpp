#include <bits/stdc++.h>
using namespace std;
typedef int64_t ll;
const int INF = 900000;
const ll INFL = 9000000;

#define rep(i, n) for(int i=0; i < (n); ++i)
#define per(i, n) for(int i=(n)-1; i >= 0; --i)
#define range(i, begin, end) for(int i=(begin); i < (end); ++i)

#define ALL(vec) vec.begin(), vec.end()


void solve(long long N, std::vector<long long> T, long long M, std::vector<long long> P, std::vector<long long> X){
	ll sum = accumulate(ALL(T), (ll)0);
	rep(i, M) {
		ll diff = X[i] - T[P[i]-1];
		cout << sum + diff << endl;
	}
}

int main(){
  long long N;
		scanf("%lld",&N);
		std::vector<long long> T(N);
		for(int i = 0 ; i < N ; i++){
				scanf("%lld",&T[i]);
		}
		long long M;
		scanf("%lld",&M);
		std::vector<long long> P(M);
		std::vector<long long> X(M);
		for(int i = 0 ; i < M ; i++){
				scanf("%lld",&P[i]);
				scanf("%lld",&X[i]);
		}
  solve(N, std::move(T), M, std::move(P), std::move(X));
  return 0;
}

