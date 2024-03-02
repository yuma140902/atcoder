#include <bits/stdc++.h>
using namespace std;
typedef int64_t ll;
const int INF = 900000;
const ll INFL = 9000000;

#define rep(i, n) for (int i = 0; i < (n); ++i)
#define per(i, n) for (int i = (n)-1; i >= 0; --i)
#define range(i, begin, end) for (int i = (begin); i < (end); ++i)

#define ALL(vec) vec.begin(), vec.end()

ll min_divisor(ll x)
{
	for (int i = 2; i <= sqrt(x); ++i)
	{
		if (x % i == 0)
			return i;
	}
	return x;
}

void solve(long long N, std::vector<long long> X)
{
	set<ll> min_divisors;
	rep(i, N)
	{
		ll tmp = min_divisor(X[i]);
		min_divisors.emplace(tmp);
	}

	ll ans = 1;
	set<ll> test;
	for (auto it = min_divisors.begin(); it != min_divisors.end(); ++it)
	{
		ans *= *it;
		test.emplace(*it);
	}
	if (N != 1 && test.size() == 1)
	{
		cout << min_divisor(X[0]) + 1 << endl;
	}
	else
	{
		cout << ans << endl;
	}
}

int main()
{
	long long N;
	scanf("%lld", &N);
	std::vector<long long> X(N);
	for (int i = 0; i < N; i++)
	{
		scanf("%lld", &X[i]);
	}
	solve(N, std::move(X));
	return 0;
}

