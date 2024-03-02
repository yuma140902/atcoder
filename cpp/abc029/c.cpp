#include <bits/stdc++.h>
using namespace std;
typedef int64_t ll;
const int INF = 900000;
const ll INFL = 9000000;

#define rep(i, n) for (int i = 0; i < (n); ++i)
#define per(i, n) for (int i = (n)-1; i >= 0; --i)
#define range(i, begin, end) for (int i = (begin); i < (end); ++i)

#define ALL(vec) vec.begin(), vec.end()

char itoa(ll i)
{
  return i == 0 ? 'a' : (i == 1 ? 'b' : 'c');
}

void solve(long long N)
{
  ll max = pow(3, N);
  for (int i = 0; i < max; ++i)
  {
    int n = i;
    string s;
    while (n > 0)
    {
      s += itoa(n % 3);
      n /= 3;
    }
    for (int j = 0; j < N - s.size(); ++j)
    {
      cout << 'a';
    }
    reverse(ALL(s));
    cout << s << endl;
  }
}

int main()
{
  long long N;
  scanf("%lld", &N);
  solve(N);
  return 0;
}

