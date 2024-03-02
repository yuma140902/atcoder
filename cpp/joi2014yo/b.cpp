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
#define per(i, n) for(int i=(n)-1; i >= 0; --i)

int main(){
	int N, M;
	cin >> N;
	cin >> M;
	vi A(N);
	vi B(M);
	rep(i, N){
		int a;
		cin >> a;
		A[i] = a;
	}
	rep(j, M){
		int b;
		cin >> b;
		B[j] = b;
	}

	vi vote = vi_def(N, 0);

	rep(j, M){
		rep(i, N){
			if(B[j] >= A[i]){
				++vote[i];
				break;
			}
		}
	}

	int max = 0;
	int maxidx = -1;
	rep(i, N){
		if(max < vote[i]){
			max = vote[i];
			maxidx = i;
		}
	}

	cout << maxidx+1 << endl;
}

