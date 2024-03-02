#include <bits/stdc++.h>
using namespace std;

int main(){
  int D, N;
  cin >> D >> N;
  
  vector<int> temps(D);
  for(int i=0; i<D; ++i){
    int in;
    cin >> in;
    temps[i] = in;
  }
  
  vector<int> as(N);
  vector<int> bs(N);
  vector<int> cs(N);
  for(int j=0; j<N; ++j){
    int a, b, c;
    cin >> a >> b >> c;
    as[j] = a;
    bs[j] = b;
    cs[j] = c;
  }
  
  vector<vector<bool>> valid(D, vector<bool>(N));
  for(int i=0; i<D; ++i) for(int j=0; j<N; ++j)
    valid[i][j] = (as[j] <= temps[i] && temps[i] <= bs[j]);
  
  //dp[i][j] <=> i日目にj番目の服を選んだ時の i日目までに着る服の派手さの差の絶対値の合計の最大値
  //dp[i+1][k] は dp[i][j] + |C_k - C_j| の最大の値
  vector<vector<int>> dp(D, vector<int>(N, -1));
  for(int j=0; j<N; ++j) {
    if(valid[0][j]) dp[0][j] = 0;
  }
  for(int i=0; i<D-1; ++i) for(int j=0; j<N; ++j) if(dp[i][j] >= 0) {
    for(int k=0; k<N; ++k) if(valid[i+1][k])
      dp[i+1][k] = max(dp[i+1][k], dp[i][j] + abs(cs[k] - cs[j]));
  }
  
  /*for(int i=0; i<D; ++i) {
    for(int j=0; j<N; ++j) {
      cout << dp[i][j] << " ";
    }
    cout << endl;
  }*/
  
  int ans = 0;
  for(int j=0; j<N; ++j)
    ans = max(ans, dp[D-1][j]);
  
  cout << ans << endl;
}

