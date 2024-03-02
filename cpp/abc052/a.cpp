#include <bits/stdc++.h>
using namespace std;
typedef int64_t ll;
const int INF = 900000;
const ll INFL = 9000000;

#define rep(i, n) for(int i=0; i < (n); ++i)
#define per(i, n) for(int i=(n)-1; i >= 0; --i)
#define range(i, begin, end) for(int i=(begin); i < (end); ++i)

#define ALL(vec) vec.begin(), vec.end()


void solve(long long A, long long B, long long C, long long D){
	cout << max(A*B, C*D) << endl;
}

int main(){
  long long A;
		scanf("%lld",&A);
		long long B;
		scanf("%lld",&B);
		long long C;
		scanf("%lld",&C);
		long long D;
		scanf("%lld",&D);
  solve(A, B, C, D);
  return 0;
}

