#include<bits/stdc++.h>
using namespace std;
constexpr char UP = 'U';
constexpr char DOWN = 'D';
constexpr char LEFT ='L';
constexpr char RIGHT = 'R';

int height, width, steps;
int row_pos, col_pos;
vector<char> S;
vector<char> T;

bool apprCenter(char dir);
bool leaveCenter(char dir);
bool move(char dir);

int main(){
  cin >> height >> width >> steps;
  cin >> row_pos >> col_pos;
  --row_pos;
  --col_pos;
  S = vector<char>(steps);
  T = vector<char>(steps);
  for(int i=0; i<steps; ++i){
    char in;
    cin >> in;
    S[i] = in;
  }
  for(int i=0; i<steps; ++i){
    char in;
    cin >> in;
    T[i] = in;
  }
  
  string result = "YES";
  
  for(int i=0; i<steps; ++i){
    if(leaveCenter(S[i])) {
      if(!move(S[i])){
        result = "NO";
        break;
      }
    }
    if(apprCenter(T[i])) {
      if(!move(T[i])){
        result = "NO";
        break;
      }
    }
  }
  
  cout << result << endl;
}

bool apprCenter(char dir){
  int mid_height = (height+1)/2;
  int mid_width = (width+1)/2;
  if(dir == UP){
    return mid_height < row_pos+1;
  }
  else if(dir == DOWN){
    return row_pos+1 < mid_height;
  }
  else if(dir == LEFT){
    return mid_width < col_pos+1;
  }
  else{
    return col_pos+1 < mid_width;
  }
}

bool leaveCenter(char dir){
  int mid_height = (height+1)/2;
  int mid_width = (width+1)/2;
  if(dir == UP){
    return row_pos+1 <= mid_height;
  }
  else if(dir == DOWN){
    return mid_height <= row_pos+1;
  }
  else if(dir == LEFT){
    return col_pos+1 <= mid_width;
  }
  else{
    return mid_width <= col_pos+1;
  }
}

bool move(char dir){
  if(dir == UP){
    --row_pos;
  }
  else if(dir == DOWN){
    ++row_pos;
  }
  else if(dir == LEFT){
    --col_pos;
  }
  else{
    ++col_pos;
  }
  
  //cout << "moved to " << dir << ": " << row_pos << ", " << col_pos << endl;
  return (0 <= row_pos && row_pos < height) && (0 <= col_pos && col_pos < width);
}

