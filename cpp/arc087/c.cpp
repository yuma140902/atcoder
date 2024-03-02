#include <bits/stdc++.h>
using namespace std;
typedef int64_t ll;
const int INF = 900000;
const ll INFL = 9000000;

#define rep(i, n) for(int i=0; i < (n); ++i)
#define per(i, n) for(int i=(n)-1; i >= 0; --i)
#define range(i, begin, end) for(int i=(begin); i < (end); ++i)

#define ALL(vec) vec.begin(), vec.end()


void solve(long long N, std::vector<long long> a){
	sort(ALL(a));

	ll ans = 0;
	ll cnt = 1;
	range(i, 1, N) {
		if(a[i-1] != a[i]) {
			if(cnt > a[i-1]) {
				ans += (cnt - a[i-1]);
			}
			else if(cnt < a[i-1]) {
				ans += cnt;
			}
			cnt = 1;
		}
		if(a[i-1] == a[i]){
			++cnt;
		}
	}
	if(cnt > a[N-1]) {
		ans += (cnt - a[N-1]);
	}
	else if(cnt < a[N-1]) {
		ans += cnt;
	}

	cout << ans << endl;
}

int main(){
  long long N;
		scanf("%lld",&N);
		std::vector<long long> a(N);
		for(int i = 0 ; i < N ; i++){
				scanf("%lld",&a[i]);
		}
  solve(N, std::move(a));
  return 0;
}

