#include <bits/stdc++.h>
using namespace std;
typedef int64_t ll;
const int INF = 900000;
const ll INFL = 9000000;

#define rep(i, n) for(int i=0; i < (n); ++i)
#define per(i, n) for(int i=(n)-1; i >= 0; --i)
#define range(i, begin, end) for(int i=(begin); i < (end); ++i)

#define ALL(vec) vec.begin(), vec.end()


void solve(long long A, std::string op, long long B){
	cout << (op == "+" ? A+B : A-B) << endl;
}

int main(){
  long long A;
		scanf("%lld",&A);
		std::string op;
		std::cin >> op;
		long long B;
		scanf("%lld",&B);
  solve(A, op, B);
  return 0;
}

