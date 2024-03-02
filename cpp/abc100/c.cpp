#include <bits/stdc++.h>
using namespace std;
typedef long long int ll; // Long long
typedef ll nt; // Number type

#define rep(i, n) for(int i=0; i < (n); ++i)
#define per(i, n) for(int i=(n)-1 i >= 0; --i)

int main(){
	int N;
	vector<int> a;
	cin >> N;
	a = vector<int>(N);
	rep(i, N){
		int in;
		cin >> in;
		a[i] = in;
	}

	int maxExp = 0;
	rep(i, N){
		int exp = 0;
		while((a[i]&1) == 0){
			++exp;
			a[i] = a[i]/2;
		}
		maxExp += exp;
	}

	cout << maxExp << endl;
}

