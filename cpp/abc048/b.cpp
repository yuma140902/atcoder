#include <bits/stdc++.h>
using namespace std;
typedef int64_t ll;
const int INF = 900000;
const ll INFL = 9000000;

#define rep(i, n) for(int i=0; i < (n); ++i)
#define per(i, n) for(int i=(n)-1; i >= 0; --i)
#define range(i, begin, end) for(int i=(begin); i < (end); ++i)

#define ALL(vec) vec.begin(), vec.end()


void solve(long long a, long long b, long long x){
	ll l = (a+x-1)/x*x;
	ll r = b-b%x;
	if(l > r) cout << 0 << endl;
	else cout << (r/x-l/x+1) << endl;
}

int main(){
  long long a;
		scanf("%lld",&a);
		long long b;
		scanf("%lld",&b);
		long long x;
		scanf("%lld",&x);
  solve(a, b, x);
  return 0;
}

