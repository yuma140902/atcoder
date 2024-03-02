#include <bits/stdc++.h>
using namespace std;
typedef int64_t ll;
const int INF = 900000;
const ll INFL = 9000000;

#define rep(i, n) for(int i=0; i < (n); ++i)
#define per(i, n) for(int i=(n)-1; i >= 0; --i)
#define range(i, begin, end) for(int i=(begin); i < (end); ++i)

#define ALL(vec) vec.begin(), vec.end()

const string YES = "YES";
const string NO = "NO";

bool endsWith(string s, string t){
  return s.size() >= t.size() && s.find(t, s.size() - t.size()) != std::string::npos;
}

void solve(std::string S){
  string surfixes[] = {
    "dream", "dreamer", "erase", "eraser"
  };
  while(S.size() > 0) {
    bool match = false;
    rep(i, 4){
      if(endsWith(S, surfixes[i])){
        //cout << "S: " << S << ", surfix: " << surfixes[i] << ", ";
        //cout << "substr: " << S.substr(S.size() - surfixes[i].size()) << endl;
        S = S.substr(0, S.size() - surfixes[i].size());
        match = true;
      }
    }
    if(!match) break;
  }

  cout << ((S.size() == 0) ? YES : NO) << endl;
}

int main(){
  std::string S;
		std::cin >> S;
  solve(S);
  return 0;
}

