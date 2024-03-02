#include<bits/stdc++.h>
using std::vector;
using std::cerr;
using std::endl;

int N, M;
vector<vector<int>> G;

int main() {
  std::cin >> N >> M;
  G = vector<vector<int>>(N, vector<int>());
  for(int i=0; i<M; ++i) {
    int a, b;
    std::cin >> a >> b;
    G[a-1].push_back(b);
  }
  
  std::queue<int> Q {};
  vector<int> cnt(N, 0);
  for(int s=1; s<=N; ++s) {
    vector<bool> visit(N, false);
    visit[s-1] = true;
    Q.push(s);
 	while(!Q.empty()) {
 	  auto c = Q.front();
 	  Q.pop();
      for(int n : G[c-1]) {
        if(!visit[n-1]) {
          visit[n-1] = true;
          Q.push(n);
        }
      }
  	}
    for(int i=0; i<visit.size(); ++i) if(visit[i]) ++cnt[s-1];
  }
  
  int sum = 0;
  for(int i=0; i<cnt.size(); ++i) sum += cnt[i];
  std::cout << sum << std::endl;
}
