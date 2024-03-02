#include <bits/stdc++.h>
#include <cstdio>
using namespace std;
typedef long long int ll; // Long long

#define vi vector<int>
#define vii vector<vector<int> >
#define vi_(n) vector<int>((n))
#define vii_(n,m) vector<vector<int> >((n), vector<int>((m)))
#define vi_def(n, def) vector<int>((n), (def))
#define vii_def(n, m, def) vector<vector<int> >((n), vector<int>((m), (def)))

#define rep(i, n) for(int i=0; i < (n); ++i)
#define per(i, n) for(int i=(n)-1 i >= 0; --i)

int main(){
	int N;
	cin >> N;
	vi A(N+1);
	vi B(N);
	vi C = vi_def(N+1, 0);

	rep(i, N+1){
		int a;
		cin >> a;
		A[i] = a;
	}
	rep(i, N){
		int b;
		cin >> b;
		B[i] = b;
	}

	rep(i, N){
		int tmp = min(A[i]-C[i], B[i]);
		C[i] += tmp;
		B[i] -= tmp;
		C[i+1] += min(A[i+1]-C[i+1], B[i]);
	}

	int ans = 0;
	rep(i, N+1){
		ans += C[i];
	}

	cout << ans << endl;

}

