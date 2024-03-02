#include <bits/stdc++.h>
using namespace std;
typedef int64_t ll;
const int INF = 900000;
const ll INFL = 9000000;

#define rep(i, n) for(int i=0; i < (n); ++i)
#define per(i, n) for(int i=(n)-1; i >= 0; --i)
#define range(i, begin, end) for(int i=(begin); i < (end); ++i)

#define ALL(vec) vec.begin(), vec.end()

ll at(vector<long long> &V, size_t index) {
	if(index < 0 || V.size() <= index){
		return -INFL;
	}
	else return V[index];
}

void solve(long long X, long long Y, long long Z, long long K, std::vector<long long> A, std::vector<long long> B, std::vector<long long> C){
	sort(ALL(A));
	sort(ALL(B));
	sort(ALL(C));

	//priority_queue<ll> queue;
	priority_queue<ll, vector<ll>, greater<ll> > queue;

	rep(x, X) {
		rep(y, Y) {
			rep(z, Z) {
				ll tmp = A[x] + B[y] + C[z];
				if(queue.size() <= K)
					queue.push(tmp);
				else{
					if(queue.top() < tmp){
						queue.pop();
						queue.push(tmp);
					}
				}
			}
		}
	}

	vector<ll> v(K);
	rep(i, K){
		v[i] = queue.top();
		queue.pop();
	}

	rep(i, K){
		cout << v[i] << endl;
	}
}

int main(){
  long long X;
		scanf("%lld",&X);
		long long Y;
		scanf("%lld",&Y);
		long long Z;
		scanf("%lld",&Z);
		long long K;
		scanf("%lld",&K);
		std::vector<long long> A(X);
		for(int i = 0 ; i < X ; i++){
				scanf("%lld",&A[i]);
		}
		std::vector<long long> B(Y);
		for(int i = 0 ; i < Y ; i++){
				scanf("%lld",&B[i]);
		}
		std::vector<long long> C(Z);
		for(int i = 0 ; i < Z ; i++){
				scanf("%lld",&C[i]);
		}
  solve(X, Y, Z, K, std::move(A), std::move(B), std::move(C));
  return 0;
}

