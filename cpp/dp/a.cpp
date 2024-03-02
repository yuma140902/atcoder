#include <bits/stdc++.h>
using namespace std;
typedef long long int ll; // Long long
const int INF = 90000;
const ll INFL = 900000;

#define vi vector<int>
#define vii vector<vector<int> >
#define vi_(n) vector<int>((n))
#define vii_(n,m) vector<vector<int> >((n), vector<int>((m)))
#define vi_def(n, def) vector<int>((n), (def))
#define vii_def(n, m, def) vector<vector<int> >((n), vector<int>((m), (def)))

#define rep(i, n) for(int i=0; i < (n); ++i)
#define per(i, n) for(int i=(n)-1; i >= 0; --i)
#define range(i, begin, end) for(int i=(begin); i < (end); ++i)

int N;
vi H;
vi dp;

int main(){
	cin >> N;
	H = vi(N);
	rep(i, N){
		int h;
		cin >> h;
		H[i] = h;
	}

	dp = vi(N); // dp[i] := 足場0から足場iまでの最小のコスト
	dp[0] = 0;
	dp[1] = abs(H[0] - H[1]);
	range(i, 2, N){
		int c1 = abs(H[i] - H[i-1]) + dp[i-1];
		int c2 = abs(H[i] - H[i-2]) + dp[i-2];
		dp[i] = min(c1, c2);
	}

	int ans = dp[N-1];
	cout << ans << endl;
}

