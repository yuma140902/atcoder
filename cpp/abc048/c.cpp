#include <bits/stdc++.h>
using namespace std;
typedef int64_t ll;
const int INF = 900000;
const ll INFL = 9000000;

#define rep(i, n) for(int i=0; i < (n); ++i)
#define per(i, n) for(int i=(n)-1; i >= 0; --i)
#define range(i, begin, end) for(int i=(begin); i < (end); ++i)

#define ALL(vec) vec.begin(), vec.end()


void solve(long long N, long long x, std::vector<long long> a){
	ll ans = 0;
	if(a[0] > x) {
		ans += a[0] - x;
		a[0] = x;
	}
	rep(i, N-1){
		if(a[i] + a[i+1] > x) {
			ll eat = a[i]+a[i+1]-x;
			a[i+1] -= eat;
			ans += eat;
		}
	}
	cout << ans << endl;
}

int main(){
  long long N;
		scanf("%lld",&N);
		long long x;
		scanf("%lld",&x);
		std::vector<long long> a(N);
		for(int i = 0 ; i < N ; i++){
				scanf("%lld",&a[i]);
		}
  solve(N, x, std::move(a));
  return 0;
}

